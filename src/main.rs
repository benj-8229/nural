mod cli;
mod config;
mod config_serialize;
mod commands;
mod note_query;

use std::io::{Error, ErrorKind};
use config::Config;
use cli::{parse_cli, Commands};
use commands::ICommand;

fn main() {
    let cli = parse_cli();
    
    // Auto generate config directory and config file if it doesn't exist
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
    let result = match cli.subcommand {
        Commands::Init { .. } => commands::init::InitCommand::execute(config, cli),
        Commands::Create { .. } => commands::create::CreateCommand::execute(config, cli),
        Commands::Append { .. } => commands::append::AppendCommand::execute(config, cli),
        _ => Ok(()),
    };
    
    match result {
        Ok(_) => {},
        Err(e) => { println!("{}", e.to_string()); },
    };
}
