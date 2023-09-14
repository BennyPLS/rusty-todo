#![deny(missing_docs)]
//!
//!
//!
//!
//!

use std::path::PathBuf;
use std::{io, process};

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use serde_any::Format::{Json, Toml, Xml, Yaml};
use todo::storage::{export_file, import_file};
use todo::tasks::Tasks;
use todo::{config, tasks};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all tasks
    List {
        /// Print in a one line for task
        #[arg(long, short)]
        short: bool,
    },

    /// Add a new Task
    Add {
        /// Name of the task
        name: String,

        /// Description
        #[arg(long, short)]
        description: Option<String>,
    },

    /// To remove a task
    Remove {
        /// The task number
        number: usize,
    },

    /// Remove all tasks
    Clean {},

    /// Toggles the state of completed for a task
    Toggle {
        /// The task number
        number: usize,
    },

    #[command(subcommand)]
    /// Configure configuration with CLI
    Config(ConfigCommands),

    #[command(subcommand)]
    /// Export Tasks as another formats
    Convert(ConvertCommands),
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Modify the path where the data is stored, put None to reset to default
    DataPath {
        /// The path to the data
        path: Option<PathBuf>,
    },
}

#[derive(Clone, ValueEnum)]
pub enum ConvertAction {
    Import,
    Export,
}

#[derive(Subcommand)]
pub enum ConvertCommands {
    /// Convert from TOML to JSON
    JSON {
        /// The action to do
        action: ConvertAction,

        /// The path to the data
        path: PathBuf,
    },

    /// Convert from TOML to YAML
    YAML {
        /// The action to do
        action: ConvertAction,

        /// The path to the data
        path: PathBuf,
    },

    /// Convert from TOML to XML
    XML {
        /// The action to do
        action: ConvertAction,

        /// The path to the data
        path: PathBuf,
    },

    /// Convert from TOML to TOML
    TOML {
        /// The action to do
        action: ConvertAction,

        /// The path to the data
        path: PathBuf,
    },
}

macro_rules! load_and_export {
    ($config_path: expr, $target_path: expr, $format: ident) => {
        let tasks = tasks::io::load(&$config_path);

        if let Err(_) = export_file(&tasks, $format, &$target_path) {
            eprintln!("{} : Could not export to specified file.", "ERROR".red());
            process::exit(exitcode::IOERR);
        }
    };
}

macro_rules! import_and_load {
    ($config_path: expr, $target_path: expr, $format: ident) => {
        let result: io::Result<Tasks> = import_file(&$target_path, $format);

        match result {
            Ok(tasks) => {
                tasks::io::save(&tasks, &$config_path);
            }
            Err(_) => {
                eprintln!("{} : Could not import to specified file.", "ERROR".red());
                process::exit(exitcode::IOERR);
            }
        }
    };
}

macro_rules! convert {
    ($action: ident, $config_path: expr, $target_path: expr, $format: ident) => {
        match $action {
            ConvertAction::Import => {
                import_and_load!($config_path, $target_path, $format);
            }
            ConvertAction::Export => {
                load_and_export!($config_path, $target_path, $format);
            }
        }
    };
}

fn config_commands(command: ConfigCommands) {
    let mut config = config::io::load();

    config = match command {
        ConfigCommands::DataPath { path } => {
            config.set_data_path(path);
            config.validate()
        }
    };

    config::io::save(&config)
}

fn convert_commands(command: ConvertCommands) {
    let config = config::io::load();

    match command {
        ConvertCommands::TOML { path, action } => {
            convert!(action, config.get_data_path(), path, Toml)
        }
        ConvertCommands::JSON { path, action } => {
            convert!(action, config.get_data_path(), path, Json)
        }
        ConvertCommands::YAML { path, action } => {
            convert!(action, config.get_data_path(), path, Yaml)
        }
        ConvertCommands::XML { path, action } => {
            convert!(action, config.get_data_path(), path, Xml)
        }
    };

    config::io::save(&config)
}
fn tasks_commands(commands: Commands) {
    let config = config::io::load();
    let mut tasks = tasks::io::load(&config.get_data_path());

    match commands {
        Commands::List { short } => {
            if short {
                tasks.list_short();
            } else {
                tasks.list_long();
            }
        }

        Commands::Add { name, description } => {
            tasks.add(&name, &description);
        }

        Commands::Remove { number } => {
            tasks.remove(number);
        }

        Commands::Toggle { number } => {
            tasks.toggle_completed(number);
        }

        Commands::Clean { .. } => {
            tasks = Tasks::new();
        }

        _ => {}
    }

    tasks::io::save(&tasks, &config.get_data_path());
}

pub fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config(command) => {
            config_commands(command);
        }

        Commands::Convert(command) => {
            convert_commands(command);
        }

        _ => tasks_commands(cli.command),
    }
}
