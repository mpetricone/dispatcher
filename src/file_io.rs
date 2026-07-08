//! File IO
//! We'll be storing to json whenever possible
use crate::normalize::Normalizer;
use serde;
use serde::de;
use serde_json;
use std::error::Error;
use std::fs::{OpenOptions, read_dir};
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
///
/// This is designed to read any struct implementing [serde::Deserialize]
/// from file in json format.
///
pub fn from_file<T: de::DeserializeOwned>(file_path: &str) -> Result<T, Box<dyn Error>> {
    let fh = OpenOptions::new().read(true).write(false).open(file_path)?;
    let bufr = BufReader::new(fh);
    let ret_v: T = serde_json::from_reader(bufr)?;
    Ok(ret_v)
}

/// Writes any struct implementing [serde::Serialize] to file, with the option to truncate.
///
/// to_file will always attempt to create a file if none exists.
pub fn to_file<T: serde::Serialize + Normalizer>(
    file_path: &str,
    truncate: bool,
    data: &mut T,
) -> Result<(), Box<dyn Error>> {
    let fh = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(truncate)
        .open(file_path)?;
    let bufw = BufWriter::new(fh);
    data.normalize();
    Ok(serde_json::to_writer(bufw, &data)?)
}

pub fn get_dir_list(path: &Path) -> io::Result<Vec<PathBuf>> {
    read_dir(path)?
        .map(|entries| entries.map(|e| e.path()))
        .collect()
}

pub fn get_file_list_recursive(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut ret_v = Vec::new();
    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                ret_v.extend(get_file_list_recursive(&entry.path())?);
            } else {
                ret_v.push(entry.path());
            }
        }
    }
    Ok(ret_v)
}
