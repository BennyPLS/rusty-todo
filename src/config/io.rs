use std::path::PathBuf;
use std::{io, process};

use colored::Colorize;
use serde_any::Format;

use crate::config::Config;
use crate::storage::{export_file, import_file};

const CONFIG_FILE_NAME: &str = "todo.config";

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

pub fn save(config: &Config) {
    if let Err(err) = export_file(config, Format::Toml, &default_config_path()) {
        eprintln!("{} : Config file, {}", "ERROR".red(), err.to_string());
        process::exit(exitcode::IOERR);
    }
}

pub fn load() -> Config {
    let result: io::Result<Config> = import_file(&default_config_path(), Format::Toml);

    match result {
        Ok(data) => data.validate(),
        Err(err) => {
            eprintln!("{} : Config file, {}", "ERROR".red(), err.to_string());
            process::exit(exitcode::IOERR);
        }
    }
}
