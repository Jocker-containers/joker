use std::collections::{HashMap};
use std::fs::File;
use std::net::SocketAddr;
use std::io;
use serde::{Serialize, Deserialize};

pub const PATH: &str = "config.cfg";

#[derive(Serialize, Deserialize)]
pub struct Daemon {
    pub name: String,
    pub socket_address: SocketAddr,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub current_daemon: Daemon,
    pub daemons: HashMap<String, SocketAddr>,
}

pub fn get_config() -> Result<Config, io::Error> {
    let config_file = File::open(PATH)?;

    let config: Config = serde_json::from_reader(config_file)?;

    Ok(config)
}

pub fn write_config(config: &Config) -> Result<(), io::Error> {
    let config_file = File::create(PATH)?;

    serde_json::to_writer(config_file, config)?;

    Ok(())
}
