use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::read_to_string,
    net::SocketAddr,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Server {
    pub address: Option<SocketAddr>,
    pub base: Option<String>,
    pub static_path: Option<PathBuf>,
    pub index: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub logging: log4rs::file::RawConfig,
    pub server: Server,
}

impl Config {
    pub fn parse<P: AsRef<Path>>(
        path: P,
    ) -> Result<(Config, log4rs::config::Config), Box<dyn Error>> {
        let config = read_to_string(path)?;

        let config: Config = serde_yaml::from_str(&config)?;

        let config_deserializers = log4rs::file::Deserializers::new();
        let (appenders, errors) = config.logging.appenders_lossy(&config_deserializers);
        for error in &errors {
            eprintln!("log4rs: {}", error);
        }
        let (log4rs_config, errors) = log4rs::config::Config::builder()
            .appenders(appenders)
            .loggers(config.logging.loggers())
            .build_lossy(config.logging.root());
        for error in &errors {
            eprintln!("log4rs: {}", error);
        }

        Ok((config, log4rs_config))
    }
}
