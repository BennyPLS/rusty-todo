use std::path::PathBuf;
use std::{io, process};

use crate::libs::config::Config;
use crate::libs::storage::{export_file, import_file_or_create};
use colored::Colorize;
use serde_any::Format;

const CONFIG_FILE_NAME: &str = "todo.config";

/// A module to handle the configuration of the application.

/// `default_config_path` returns the default config path.
pub(crate) fn default_config_path() -> PathBuf {
    match dirs::config_dir() {
        Some(mut path) => {
            path.push(CONFIG_FILE_NAME);
            path
        }

        None => {
            eprintln!("{} : Config directory not found.", "ERROR".red());
            process::exit(exitcode::IOERR);
        }
    }
}

/// `save` saves the configuration to the default config path.
pub fn save(config: &Config) {
    if let Err(err) = export_file(config, Format::Toml, &default_config_path()) {
        eprintln!("{} : Config file, {}", "ERROR".red(), err);
        process::exit(exitcode::IOERR);
    }
}

/// `load` loads the configuration from the default config path.
pub fn load() -> Config {
    let result: io::Result<Config> = import_file_or_create(&default_config_path(), Format::Toml);

    match result {
        Ok(data) => data.validate(),
        Err(err) => {
            eprintln!("{} : Config file, {}", "ERROR".red(), err);
            process::exit(exitcode::IOERR);
        }
    }
}
