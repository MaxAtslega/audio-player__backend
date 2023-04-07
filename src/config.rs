use clap::{App, Arg};
use rocket::log::LogLevel;
use serde_derive::Deserialize;
use std::net::IpAddr;
use std::{env, fs};
use thiserror::Error;

use log::{debug, error};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app: AppConf,
    pub database: DatabaseConfig,
    pub webserver: WebServerConf,
    pub log: LogConf,
}

#[derive(Deserialize, Debug)]
pub struct LogConf {
    pub file: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConf {
    pub environment: String,
}

#[derive(Deserialize, Debug)]
pub struct WebServerConf {
    pub address: IpAddr,
    pub port: u16,
    pub ident: String,
    pub log_level: LogLevel,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub db_type: String,
    pub connection_string: String,
}

#[derive(Error, Debug)]
pub enum ConfigError{
    #[error("Config File could not be found")]
    ConfigNotFound(std::io::Error),

    #[error("Config File could not be found")]
    ParsingError(toml::de::Error),

    #[error("Env variable could not be found")]
    EnvVarNotFound(std::env::VarError),

    #[error("Cli args could not be found")]
    CliArgsNotFound,

}

impl Config{
    pub fn from_any() -> Result<Self, ConfigError>{
        //Try to read from path cli args
        let cli_result = Self::from_cli_args();
        match cli_result{
            Ok(config) => {
                debug!("Loaded config from cli path");
                return Ok(config);
            }
            Err(error) => {
                debug!("Could not load config from env path: {}", error);
            }
        }

        //Try to read from path env var
        let env_result = Self::from_env_path();
        match env_result{
            Ok(config) => {
                debug!("Loaded config from env path");
                return Ok(config);
            }
            Err(error) => {
                debug!("Could not load config from env path: {}", error);
            }
        }

        //Try to read default path
        let default_result = Self::from_default_path();

        match default_result {
            Ok(config) => {
                debug!("Loaded config from default path");
                Ok(config)
            }
            Err(error) => {
                error!("Could not load config: {}", error);
                Err(error)
            }
        }
    }

    //Read Config from default path
    pub fn from_default_path() -> Result<Self, ConfigError>{
        let path = "config.toml";
        Self::from_file_path(&path)
    }

    //Read Config from path in CONFIG_LOCATION env variable
    pub fn from_env_path() -> Result<Self, ConfigError>{
        let path = env::var("CONFIG_LOCATION")
            .map_err(|e|ConfigError::EnvVarNotFound(e))?;
        Self::from_file_path(&path)
    }

    //Read Config from path in config / c cli args
    pub fn from_cli_args() -> Result<Self, ConfigError>{
        let args = App::new("AudioPlayer Backend")
            .version("0.1.0")
            .about("Backend for AudioPlayer")
            .author("Max Atslega")
            .args(&[Arg::new("config")
                .short('c')
                .long("config")
                .takes_value(true)])
            .get_matches();

        if args.is_present("config") {
            let path: String = args
                .value_of_t("config")
                .expect("Failed to read config path");
            Self::from_file_path(&path)
        }else{
            Err(ConfigError::CliArgsNotFound)
        }
    }

    //Read and Parse Config from path
    pub fn from_file_path(path: &str)->Result<Self, ConfigError>{
        let data = fs::read_to_string(path)
            .map_err(|e|ConfigError::ConfigNotFound(e))?;

        toml::from_str(data.as_str())
            .map_err(|e|ConfigError::ParsingError(e))
    }
}