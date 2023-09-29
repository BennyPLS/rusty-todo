mod conversion;

use crate::conversion::{convert_commands, ConvertCommand};
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use todo::libs::tasks::Tasks;
use todo::libs::{config, tasks};

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
    Clean,

    /// Toggles the state of completed for a task
    Toggle {
        /// The task number
        number: usize,
    },

    #[command(subcommand)]
    /// Configure configuration with CLI
    Config(ConfigCommands),

    /// Export Tasks as another formats
    Convert {
        #[command(flatten)]
        args: ConvertCommand,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Modify the path where the data is stored, put None to reset to default
    DataPath {
        /// The path to the data
        path: Option<PathBuf>,
    },
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

    print!("{} ", "TODO".green());

    match cli.command {
        Commands::Config(command) => {
            config_commands(command);
        }

        Commands::Convert { args } => convert_commands(args),

        _ => tasks_commands(cli.command),
    }
}
