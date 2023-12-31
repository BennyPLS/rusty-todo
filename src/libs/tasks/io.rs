use std::path::PathBuf;
use std::{io, process};

use crate::libs::storage::{export_file, import_file_or_create};
use crate::libs::tasks::Tasks;
use colored::Colorize;
use serde_any::Format;

/// A module to handle the tasks of the application.

/// `default_data_path` returns the default data path.
pub fn save(tasks: &Tasks, path: &PathBuf) {
    if let Err(err) = export_file(tasks, Format::Toml, &path) {
        eprintln!("{} : Config file, {}", "ERROR".red(), err.to_string());
        process::exit(exitcode::IOERR);
    };
}

/// `load` loads the tasks from the default data path.
pub fn load(path: &PathBuf) -> Tasks {
    let result: io::Result<Tasks> = import_file_or_create(&path, Format::Toml);

    match result {
        Ok(tasks) => tasks,
        Err(err) => {
            eprintln!("{} : Data file, {}", "Warning".red(), err.to_string());
            process::exit(exitcode::IOERR);
        }
    }
}
