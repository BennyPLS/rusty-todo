use std::path::PathBuf;
use std::process;

use colored::Colorize;
use serde::{Deserialize, Serialize};

pub mod io;

const DATA_FILE_NAME: &str = "task.list";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    data_path_file: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            data_path_file: None,
        }
    }
}

impl Config {
    pub fn set_data_path(&mut self, new_path: Option<PathBuf>) {
        self.data_path_file = new_path;
    }

    pub fn get_data_path(&self) -> PathBuf {
        match &self.data_path_file {
            Some(path) => path.clone(),
            None => Self::default_data_path(),
        }
    }

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
