//! File IO
//! We'll be storing to json whenever possible
use serde;
use serde::de;
use serde_json;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use crate::normalize::Normalizer;
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
