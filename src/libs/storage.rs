use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::{io, process};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_any::Format;

fn create(path: &PathBuf) -> File {
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

pub(crate) fn raw_save(data: &str, path: &PathBuf) -> io::Result<()> {
    let mut file = create(path);
    file.write_all(data.as_bytes())?;

    Ok(())
}

pub(crate) fn load_raw(path: &PathBuf) -> io::Result<String> {
    let mut contents = String::new();
    let mut file = open(path);

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

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

pub fn import_file<T>(path: &PathBuf, format: Format) -> io::Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let data = load_raw(path)?;
    Ok(import(&data, format))
}

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

pub fn export_file<T>(data: &T, format: Format, path: &PathBuf) -> io::Result<()>
where
    T: Serialize,
{
    let data = export(data, format);
    raw_save(&data, path)
}
