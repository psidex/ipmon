use std::fs::File;

use serde::Deserialize;
use serde_yaml;

#[derive(Deserialize)]
pub struct NotificationInfo {
    pub title: String,
    pub body: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: String,
    pub interval: u64,
    pub notifications: Vec<NotificationInfo>,
}

pub fn load_config(path: &str) -> Config {
    serde_yaml::from_reader(
        File::open(path).expect(&format!("Failed to open config file at {}", path)),
    )
    .expect("Failed to parse config")
}
