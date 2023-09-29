use std::path::PathBuf;
use std::process;

use colored::Colorize;
use serde::{Deserialize, Serialize};

pub mod io;

const DATA_FILE_NAME: &str = "task.list";

/// A module to handle the configuration of the application.

/// `Config` is a struct that contains the configuration of the application.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    data_path_file: Option<PathBuf>,
}

/// `Default` is an implementation of the default configuration.
/// It sets the default data path to the default data directory.
impl Default for Config {
    fn default() -> Self {
        Config {
            data_path_file: None,
        }
    }
}

/// `Config` is an implementation of the configuration.
impl Config {
    /// `set_data_path` sets the data path to the given path.
    /// If the path is None, it will be set to the default data directory.
    pub fn set_data_path(&mut self, new_path: Option<PathBuf>) {
        self.data_path_file = new_path;
    }

    /// `get_data_path` returns the data path.
    /// If the path is None, it will be set to the default data directory.
    pub fn get_data_path(&self) -> PathBuf {
        match &self.data_path_file {
            Some(path) => path.clone(),
            None => Self::default_data_path(),
        }
    }

    /// `validate` validates the configuration.
    /// If the data path is not absolute, it will exit with CONFIG error code.
    pub fn validate(self) -> Self {
        if let Some(path) = &self.data_path_file {
            if !path.is_absolute() {
                eprintln!("{} : Data path cannot be relative.", "CONFIG - ERROR".red());
                process::exit(exitcode::CONFIG);
            }

            if path.is_dir() {
                eprintln!(
                    "{} : Data path cannot be a directory.",
                    "CONFIG - ERROR".red()
                );
                process::exit(exitcode::CONFIG);
            }

            if let Some(path) = path.parent() {
                if !path.is_dir() || !path.exists() {
                    eprintln!(
                        "{} : Data path has no valid parent directory.",
                        "CONFIG - ERROR".red()
                    );
                    process::exit(exitcode::CONFIG);
                }
            }
        }

        self
    }

    /// `default_data_path` returns the default data path.
    /// If the default data directory does not exist, it will exit with IO error code.
    fn default_data_path() -> PathBuf {
        match dirs::data_dir() {
            Some(mut path) => {
                path.push(DATA_FILE_NAME);
                path
            }

            None => {
                eprintln!("{} : Data Directory not found.", "ERROR".red());
                process::exit(exitcode::IOERR);
            }
        }
    }
}
