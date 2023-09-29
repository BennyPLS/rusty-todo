use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::{io, process};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_any::Format;

/// A module to handle the storage of tasks
/// in the default config format (TOML).
/// It contains functions to load and save tasks.
/// It also contains functions to import and export tasks
/// from and to other formats.

/// `create` creates a file in the given path.
/// If the file already exists, it will be overwritten.
/// If the file does not exist, it will be created.
/// If the file cannot be created, the program will exit.
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use todo::libs::storage::create;
/// let path = PathBuf::from("data/tasks.toml");
///
/// let file = create(&path);
/// ```
pub fn create(path: &PathBuf) -> File {
    match File::create(&path) {
        Ok(fs) => fs,
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!(
                    "{} : Could not create {:?} file, permission denied.",
                    "ERROR".red(),
                    &path.to_str().unwrap_or("{unknown route}")
                );
                process::exit(exitcode::IOERR);
            }

            _ => {
                eprintln!(
                    "{} : Could not create {:?} file, {}.",
                    "ERROR".red(),
                    &path.to_str().unwrap_or("{unknown route}"),
                    error.kind()
                );
                process::exit(exitcode::IOERR);
            }
        },
    }
}

/// `open` opens a file in the given path.
/// If the file does not exist, the program will exit.
/// If the file cannot be opened, the program will exit.
pub(crate) fn open(path: &PathBuf) -> File {
    match File::open(&path) {
        Ok(fs) => fs,
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!(
                    "{} : Could not open {:?} file, permission denied.",
                    "ERROR".red(),
                    &path.to_str().unwrap_or("{unknown route}")
                );
                process::exit(exitcode::IOERR);
            }

            _ => {
                eprintln!(
                    "{} : Could not open {:?} file, {}.",
                    "ERROR".red(),
                    &path.to_str().unwrap_or("{unknown route}"),
                    error.kind()
                );
                process::exit(exitcode::IOERR);
            }
        },
    }
}

/// `raw_save` saves a string to a file in the given path.
/// If the file already exists, it will be overwritten.
/// If the file does not exist, it will be created.
/// If the file cannot be created, the program will exit.
pub(crate) fn raw_save(data: &str, path: &PathBuf) -> io::Result<()> {
    let mut file = create(path);
    file.write_all(data.as_bytes())?;

    Ok(())
}

/// `load_raw` loads a string from a file in the given path.
/// If the file does not exist, the program will exit.
pub(crate) fn load_raw(path: &PathBuf) -> io::Result<String> {
    let mut contents = String::new();
    let mut file = open(path);

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

/// `load_raw_or_create` loads a string from a file in the given path.
/// If the file does not exist, it will be created.
/// If the file cannot be created, the program will exit.
pub(crate) fn load_raw_or_create(path: &PathBuf) -> io::Result<String> {
    match load_raw(path) {
        Ok(content) => Ok(content),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                create(&path);
                Ok(String::new())
            } else {
                Err(err)
            }
        }
    }
}

/// `import` imports a string from a given format to a struct.
/// If the string cannot be deserialized, the program will exit.
pub fn import<T>(data: &str, format: Format) -> T
where
    T: for<'de> Deserialize<'de>,
{
    match serde_any::from_str(data, format) {
        Ok(obj) => obj,
        Err(err) => {
            eprintln!("{} : Deserializing, {}", "ERROR".red(), err.to_string());
            process::exit(exitcode::DATAERR);
        }
    }
}

/// `export` exports a given object to a given format.
/// If the object cannot be serialized, the program will exit.
pub fn export<T>(data: &T, format: Format) -> String
where
    T: Serialize,
{
    match serde_any::to_string(data, format) {
        Ok(serialized) => serialized,
        Err(err) => {
            eprintln!("{} : Serializing, {}", "ERROR".red(), err.to_string());
            process::exit(exitcode::DATAERR);
        }
    }
}

/// `import_file` imports a file from a given format to a struct.
/// If the file cannot be deserialized, the program will exit.
pub fn import_file<T>(path: &PathBuf, format: Format) -> io::Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let data = load_raw(path)?;
    Ok(import(&data, format))
}

/// `import_file_or_create` imports a file from a given format to a struct.
/// If the file does not exist, it will be created.
/// If the file cannot be deserialized, the program will exit.
pub fn import_file_or_create<T>(path: &PathBuf, format: Format) -> io::Result<T>
where
    T: for<'de> Deserialize<'de> + Default,
{
    let data = load_raw_or_create(path)?;

    dbg!(&data);

    if data.is_empty() {
        return Ok(T::default());
    }

    Ok(import(&data, format))
}

/// `export_file` exports a given object to a given format.
/// If the object cannot be serialized, the program will exit.
/// If the file cannot be created, the program will exit.
pub fn export_file<T>(data: &T, format: Format, path: &PathBuf) -> io::Result<()>
where
    T: Serialize,
{
    let data = export(data, format);
    raw_save(&data, path)
}
