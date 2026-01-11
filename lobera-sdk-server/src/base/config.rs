use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: String,
    pub host: String,
    pub port: u16,
    pub gameserver_host: String,
    pub gameserver_port: u16,
}

impl Config {
    pub fn load() -> Self {
        let path = if std::path::Path::new("./sdkserver.toml").exists() {
            "./sdkserver.toml"
        } else {
            "./lobera-sdk-server/sdkserver.toml"
        };
        let content = fs::read_to_string(path).expect("Failed to read sdkserver.toml");
        toml::from_str(&content).expect("Failed to parse sdkserver.toml")
    }
}
