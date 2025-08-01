mod commands;
mod models;
mod util;

use commands::ICommand;
use util::cli::{Commands, parse_cli};
use util::config::Config;

fn main() {
    let cli = parse_cli();

    // Auto generate config directory and config file if it doesn't exist
    let config = Config::default();
    if !config.dir_exists() {
        match config.create_dir() {
            Ok(()) => println!("Created config directory {}", config.dir.to_string_lossy()),
            Err(_) => panic!(
                "failed to create config directory {}, check parent directories exist",
                config.dir.to_string_lossy()
            ),
        }
    }
    if !config.config_exists() {
        match config.regenerate_config() {
            Ok(()) => println!("Created config file {}", config.path.to_string_lossy()),
            Err(_) => panic!(
                "failed to create config file {}, check write permissions",
                config.path.to_string_lossy()
            ),
        }
    }

    let config = config.get_config();

    let result = match cli.subcommand {
        Some(Commands::Init { .. }) => commands::init::InitCommand::execute(config, cli),
        Some(Commands::Create { .. }) => commands::create::CreateCommand::execute(config, cli),
        Some(Commands::Append { .. }) => commands::append::AppendCommand::execute(config, cli),
        Some(Commands::Open { .. }) => commands::open::OpenCommand::execute(config, cli),
        Some(Commands::Delete { .. }) => commands::delete::DeleteCommand::execute(config, cli),
        Some(Commands::Read { .. }) => commands::read::ReadCommand::execute(config, cli),
        Some(Commands::List { .. }) => commands::list::ListCommand::execute(config, cli),
        Some(Commands::Config { .. }) => {
            println!("{}", Config::default_path().display().to_string());
            Ok(())
        }
        // default to open, eventually could be some type of dashboard
        None => {
            let fake_cli = util::cli::CliEntry {
                subcommand: Some(Commands::Open { name: None }),
            };
            commands::open::OpenCommand::execute(config, fake_cli)
        }
    };

    crate::util::note_query::flush_stdin();

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e.to_string());
        }
    };

    crate::util::note_query::flush_stdin();
}
