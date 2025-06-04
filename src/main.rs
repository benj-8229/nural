mod cli;
mod config;
mod config_serialize;
mod commands;

use config::Config;
use cli::{parse_cli, Commands};
use commands::ICommand;

fn main() {
    let cli = parse_cli();

    let config = Config::default();
    if !config.dir_exists() {
        match config.create_dir() {
            Ok(()) => println!("Created config directory {}", config.dir.to_string_lossy()),
            Err(_) => panic!("failed to create config directory {}, check parent directories exist", config.dir.to_string_lossy()),
        }
    }
    if !config.config_exists() {
        match config.regenerate_config() {
            Ok(()) => println!("Create config file {}", config.path.to_string_lossy()),
            Err(_) => panic!("failed to create config file {}, check write permissions", config.path.to_string_lossy()),
        }
    }

    let config = config.get_config();
    match cli.subcommand {
        Commands::Init { .. } =>  { let _ = commands::init::InitCommand::execute(config, cli); },
        _ => {},
    }
}
