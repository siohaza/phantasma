mod cli;
mod client;
mod config;
mod filter;
mod logger;
mod master_server;
mod parser;
mod server;
mod server_info;

use log::error;

use crate::config::Config;

fn main() {
    let cli = cli::parse().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let mut cfg = match cli.config_path {
        Some(ref p) => match config::load(p.as_ref()) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Failed to load config: {}", e);
                return;
            }
        },
        None => Config::default(),
    };

    if let Some(level) = cli.log_level {
        cfg.log.level = level;
    }

    if let Some(ip) = cli.listen_ip {
        cfg.server.ip = ip;
    }

    if let Some(port) = cli.listen_port {
        cfg.server.port = port;
    }

    logger::init(cfg.log.level);

    if let Err(e) = master_server::run(cfg) {
        error!("{}", e);
        std::process::exit(1);
    }
}
