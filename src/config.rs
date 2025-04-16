use std::fs;
use std::io;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::str::from_utf8;

use log::LevelFilter;
use serde::{de::Error as _, Deserialize, Deserializer};
use thiserror::Error;

pub const DEFAULT_MASTER_SERVER_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
pub const DEFAULT_MASTER_SERVER_PORT: u16 = 27010;
pub const DEFAULT_TIMEOUT: u32 = 300;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub log: LogConfig,
    #[serde(default)]
    pub server: ServerConfig,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LogConfig {
    #[serde(default = "default_log_level")]
    #[serde(deserialize_with = "deserialize_log_level")]
    pub level: LevelFilter,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    #[serde(default = "default_server_ip")]
    pub ip: IpAddr,
    #[serde(default = "default_server_port")]
    pub port: u16,
    #[serde(default)]
    pub timeout: TimeoutConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            ip: default_server_ip(),
            port: default_server_port(),
            timeout: Default::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TimeoutConfig {
    #[serde(default = "default_timeout")]
    pub challenge: u32,
    #[serde(default = "default_timeout")]
    pub server: u32,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            challenge: default_timeout(),
            server: default_timeout(),
        }
    }
}

fn default_log_level() -> LevelFilter {
    LevelFilter::Warn
}

fn default_server_ip() -> IpAddr {
    DEFAULT_MASTER_SERVER_IP
}

fn default_server_port() -> u16 {
    DEFAULT_MASTER_SERVER_PORT
}

fn default_timeout() -> u32 {
    DEFAULT_TIMEOUT
}

fn deserialize_log_level<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    parse_log_level(&s).ok_or_else(|| D::Error::custom(format!("Invalid log level: \"{}\"", s)))
}

pub fn parse_log_level(s: &str) -> Option<LevelFilter> {
    use LevelFilter as E;

    let level_filter = match s {
        _ if "off".starts_with(s) => E::Off,
        _ if "error".starts_with(s) => E::Error,
        _ if "warn".starts_with(s) => E::Warn,
        _ if "info".starts_with(s) => E::Info,
        _ if "debug".starts_with(s) => E::Debug,
        _ if "trace".starts_with(s) => E::Trace,
        _ => match s.parse::<u8>() {
            Ok(0) => E::Off,
            Ok(1) => E::Error,
            Ok(2) => E::Warn,
            Ok(3) => E::Info,
            Ok(4) => E::Debug,
            Ok(5) => E::Trace,
            _ => return None,
        },
    };
    Some(level_filter)
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let data = fs::read(path)?;
    let data_str = match from_utf8(&data) {
        Ok(str) => str,
        Err(utf8_error) => {
            return Err(Error::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                utf8_error.to_string(),
            )));
        }
    };
    let config = toml::from_str(data_str).map_err(Error::Toml)?;
    Ok(config)
}
