use std::fs;
use std::io::{self, Read, Write};

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
        let credentials: Credentials = Self::read_credentials().unwrap();

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
            .url();

        println!("Go to the following URL and authorize the application: {}", auth_url);

        print!("Enter the authorization code: ");
        io::stdout().flush()?;
        let mut auth_code = String::new();
        io::stdin().read_line(&mut auth_code)?;
        let auth_code = auth_code.trim().to_string();

        Ok(auth_code)
    }
}