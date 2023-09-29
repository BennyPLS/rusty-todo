#![deny(missing_docs)]

use clap::{Args, ValueEnum};
use colored::Colorize;
use serde_any::format::Format::{Json, Toml, Xml, Yaml};
use std::path::PathBuf;
use todo::libs::config;

/// A module to handle the conversion of tasks
/// from one format to another.
///
/// # Purpose
///
/// This macro are used for avoiding code duplication
/// from the main.rs file.

/// A macro to load a set of tasks from a given path
/// in the default config format (TOML) and export them
/// to a specified target path in a given format.
///
/// # Example
///
/// ```
/// load_and_export!("config.toml", "output.json", json);
/// ```
#[macro_export]
macro_rules! load_and_export {
    ($config_path: expr, $target_path: expr, $format: ident) => {
        let tasks = ::todo::libs::tasks::io::load(&$config_path);

        if let Err(_) = ::todo::libs::storage::export_file(&tasks, $format, &$target_path) {
            eprintln!("{} : Could not export to specified file.", "ERROR".red());
            ::std::process::exit(exitcode::IOERR);
        }
    };
}

/// A macro to load a set of tasks from a given path
/// in the given format and load them to another path
/// in the default config format (TOML).
///
/// # Example
///
/// ```
/// import_and_load!("config.toml", "input.json", json);
/// ```
#[macro_export]
macro_rules! import_and_load {
    ($config_path: expr, $target_path: expr, $format: ident) => {
        let result: ::std::io::Result<::todo::libs::tasks::Tasks> =
            ::todo::libs::storage::import_file(&$target_path, $format);

        match result {
            Ok(tasks) => {
                ::todo::libs::tasks::io::save(&tasks, &$config_path);
            }
            Err(_) => {
                eprintln!("{} : Could not import to specified file.", "ERROR".red());
                ::std::process::exit(exitcode::IOERR);
            }
        }
    };
}

/// A macro to export or import a set of tasks from a given path
/// in the default config format (TOML) and export them to a specified.
///
/// # Example
///
/// ```
/// convert!(action, "config.toml", "output.json", json);
/// ```
#[macro_export]
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

#[derive(Clone, ValueEnum)]
pub enum Formats {
    /// Convert from TOML to JSON
    JSON,
    /// Convert from TOML to YAML
    YAML,
    /// Convert from TOML to XML
    XML,
    /// Convert from TOML to TOML
    TOML,
}

#[derive(Clone, ValueEnum)]
pub enum ConvertAction {
    /// Export the tasks file
    Import,
    /// Import a new tasks file
    Export,
}

#[derive(Args)]
pub struct ConvertCommand {
    /// The action to do
    action: ConvertAction,

    /// The format to convert to
    format: Formats,

    /// The path to the data
    path: PathBuf,
}

pub(crate) fn convert_commands(
    ConvertCommand {
        action,
        path,
        format,
    }: ConvertCommand,
) {
    let config = config::io::load();

    match format {
        Formats::TOML => {
            convert!(action, config.get_data_path(), path, Toml)
        }
        Formats::JSON => {
            convert!(action, config.get_data_path(), path, Json)
        }
        Formats::YAML => {
            convert!(action, config.get_data_path(), path, Yaml)
        }
        Formats::XML => {
            convert!(action, config.get_data_path(), path, Xml)
        }
    };

    config::io::save(&config)
}
