use std::fs;
use std::net::TcpListener;
use std::process::Command;
use std::io::{Read, Write};  
use url::Url;

use oauth2::{
    AuthUrl, 
    ClientId, 
    ClientSecret, 
    TokenResponse, 
    AuthorizationCode,
    reqwest::http_client,
    RefreshToken,
    basic::BasicClient,
    RedirectUrl, 
    TokenUrl,
};
use serde::{
    Serialize,
    Deserialize,
};

use crate::app::config::get_config;

#[derive(Deserialize)]
pub struct Credentials {
    pub installed: Installed,
}

#[derive(Deserialize)]
pub struct Installed {
    pub client_id: String,
    pub client_secret: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub redirect_uris: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenInfo {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct Authentication {
    pub token_info: TokenInfo
}

impl Authentication {
    pub fn new() -> Self {
        Self {
            token_info: Self::read_token(),
        }
    }

    fn read_token() -> TokenInfo {
        let credentials: Credentials = match Self::read_credentials() {
            Ok(creds) => creds,
            Err(err) => {
                panic!("Failed to read credentials: {:?}", err);
            }
        };

        let auth_url = match AuthUrl::new(credentials.installed.auth_uri.clone()) {
            Ok(url) => url,
            Err(err) => panic!("Failed to parse AuthUrl: {:?}", err),
        };
        let token_url = match TokenUrl::new(credentials.installed.token_uri.clone()) {
            Ok(url) => url,
            Err(err) => panic!("Failed to parse TokenUrl: {:?}", err),
        };
        let redirect_url = match RedirectUrl::new(credentials.installed.redirect_uris[0].clone()) {
            Ok(url) => url,
            Err(err) => panic!("Failed to parse RedirectUrl: {:?}", err),
        };

        let client = BasicClient::new(
            ClientId::new(credentials.installed.client_id.clone()),
            Some(ClientSecret::new(credentials.installed.client_secret.clone())),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url);

        match Self::get_token(&client) {
            Ok(token) => token,
            Err(err) => panic!("Failed to retrieve token: {:?}", err),
        }
    }

    pub fn read_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
        let mut file = fs::File::open("credentials.json")?;
        let mut credentials_str = String::new();
        file.read_to_string(&mut credentials_str)?;

        let credentials: Credentials = serde_json::from_str(&credentials_str)?;
        Ok(credentials)
    }

    pub fn get_token(client: &BasicClient) -> Result<TokenInfo, Box<dyn std::error::Error>> {
        let mut token_info = if let Some(token) = Self::load_token() {
            token
        } else {
            let auth_code = Self::get_authentication_code(&client)?;

            let token = 
                client.exchange_code(AuthorizationCode::new(auth_code))
                .request(http_client)?;

            let token_info = TokenInfo {
                access_token: token.access_token().secret().clone(),
                refresh_token: token.refresh_token().unwrap().secret().clone(),
            };

            Self::save_token(&token_info)?;
            token_info
        };

        let refresh_token = RefreshToken::new(token_info.refresh_token.clone());
        let token_result = client
            .exchange_refresh_token(&refresh_token)
            .request(oauth2::reqwest::http_client)?;

        token_info.access_token = token_result.access_token().secret().clone();
        if let Some(new_refresh_token) = token_result.refresh_token() {
            token_info.refresh_token = new_refresh_token.secret().clone();
        }
        Self::save_token(&token_info)?;

        Ok(token_info)
    }

    fn save_token(token_info: &TokenInfo) -> std::io::Result<()> {
        let token_json = serde_json::to_string(token_info)?;
        fs::write("token.json", token_json)?;
        Ok(())
    }

    fn load_token() -> Option<TokenInfo> {
        if let Ok(token_json) = fs::read_to_string("token.json") {
            serde_json::from_str(&token_json).ok()
        } else {
            None
        }
    }

    fn get_authentication_code(client: &BasicClient) -> Result<String, Box<dyn std::error::Error>> {
        let (auth_url, _) = client
            .authorize_url(|| oauth2::CsrfToken::new_random())
            .add_scope(oauth2::Scope::new("https://www.googleapis.com/auth/calendar".to_string()))
            .add_scope(oauth2::Scope::new("https://www.googleapis.com/auth/tasks".to_string()))
            .url();

        println!("Go to the following URL and authorize the application: {}", auth_url);

        let browser_path = &get_config().browser_path;

        Command::new(browser_path)
            .arg(auth_url.as_str())
            .output()
            .expect("Failed to execute get_authentication_code()");

        let listener = TcpListener::bind("127.0.0.1:8080")?;
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; 1024];

            stream.read(&mut buffer)?;

            let request = String::from_utf8_lossy(&buffer);

            if let Some(line) = request.lines().next() {
                if let Some(path_start) = line.find(' ') {
                    if let Some(path_end) = line[path_start+1..].find(' ') {
                        let path = &line[path_start+1..path_start+1+path_end];

                        if let Ok(url) = Url::parse(&format!("http://localhost{}", path)) {
                            if let Some(code) = url.query_pairs().find_map(|(k, v)| {
                                if k == "code" { Some(v.into_owned()) } else { None }
                            }) {
                                let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                    Authentication completed. You can close this window.";

                                stream.write_all(response.as_bytes())?;

                                return Ok(code);
                            }
                        }
                    }
                }
            }
        }

        Err("Failed to execute get_authentication_code()".into())
    }
}