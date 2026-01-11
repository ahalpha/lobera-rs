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
}

impl Config {
    pub fn load() -> Self {
        let path = if std::path::Path::new("./gameserver.toml").exists() {
            "./gameserver.toml"
        } else {
            "./lobera-game-server/gameserver.toml"
        };
        let content = fs::read_to_string(path).expect("Failed to read gameserver.toml");
        toml::from_str(&content).expect("Failed to parse gameserver.toml")
    }
}
