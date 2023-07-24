use std::fs::File;

use serde::Deserialize;
use serde_yaml::{self};

#[derive(Deserialize)]
pub struct IpmonConfigNotificationInfo {
    pub title: String,
    pub body: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct IpmonConfig {
    pub server: String,
    pub interval: u64,
    pub notifications: Vec<IpmonConfigNotificationInfo>,
}

pub fn load_config(path: &str) -> IpmonConfig {
    let f = File::open(path).unwrap_or_else(|_| panic!("Could not open config file at {}", path));
    serde_yaml::from_reader::<File, IpmonConfig>(f).expect("Could not parse config into struct")
}
