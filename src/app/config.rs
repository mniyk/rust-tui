use config::{Config, File};
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub browser_path: String,
    pub virtualbox_path: String,
}

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| {
        let settings = Config::builder()
            .add_source(File::with_name("config.ini"))
            .build()
            .unwrap();
        let browser_path = settings.get_string("PATHS.BROWSER").unwrap();
        let virtualbox_path = settings.get_string("PATHS.VIRTUALBOX").unwrap();
        AppConfig { browser_path, virtualbox_path }
    })
}
