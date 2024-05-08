use lexopt::prelude::*;
use log::LevelFilter;
use std::net::IpAddr;
use std::process;
use thiserror::Error;

use crate::config;

const BIN_NAME: &str = env!("CARGO_BIN_NAME");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP: &str = "\
OPTIONS:
  -h, --help            Print usage help
  -v, --version         Print program version
  -l, --log LEVEL       Set the logging level
  -i, --ip IP           Set the listen IP address
  -p, --port PORT       Set the listen port
  -c, --config PATH     Set the config path
";

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid ip address \"{0}\"")]
    InvalidIp(String),
    #[error("Invalid port number \"{0}\"")]
    InvalidPort(String),
    #[error(transparent)]
    Options(#[from] lexopt::Error),
}

#[derive(Debug, Default)]
pub struct Cli {
    pub log_level: Option<LevelFilter>,
    pub listen_ip: Option<IpAddr>,
    pub listen_port: Option<u16>,
    pub config_path: Option<Box<str>>,
}

pub fn parse() -> Result<Cli, Error> {
    let mut cli = Cli::default();

    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                print!("USAGE: {} [options]\n\n{}", BIN_NAME, HELP);
                process::exit(0);
            }
            Short('v') | Long("version") => {
                println!("{} v{}", PKG_NAME, PKG_VERSION);
                process::exit(0);
            }
            Short('l') | Long("log") => {
                let value = parser
                    .value()?
                    .into_string()
                    .map_err(|_| Error::Options("Failed to parse log level option".into()))?;
                if let Some(level) = config::parse_log_level(&value) {
                    cli.log_level = Some(level);
                } else {
                    eprintln!("Invalid value for log option: \"{}\"", value);
                    process::exit(1);
                }
            }
            Short('i') | Long("ip") => {
                let s = parser
                    .value()?
                    .into_string()
                    .map_err(|_| Error::Options("Failed to parse IP address option".into()))?;
                cli.listen_ip = Some(s.parse().map_err(|_| Error::InvalidIp(s))?);
            }
            Short('p') | Long("port") => {
                let s = parser
                    .value()?
                    .into_string()
                    .map_err(|_| Error::Options("Failed to parse port number option".into()))?;
                cli.listen_port = Some(s.parse().map_err(|_| Error::InvalidPort(s))?);
            }
            Short('c') | Long("config") => {
                let s = parser
                    .value()?
                    .into_string()
                    .map_err(|_| {
                        Error::Options("Failed to parse configuration path option".into())
                    })?
                    .into_boxed_str();
                cli.config_path = Some(s);
            }
            _ => return Err(arg.unexpected().into()),
        }
    }

    Ok(cli)
}
