use serde;
use serde::de;
use serde_json;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};

pub fn from_file<T: de::DeserializeOwned>(file_path: &str) -> Result<T, Box<dyn Error>> {
    let fh = OpenOptions::new().read(true).write(false).open(file_path)?;
    let bufr = BufReader::new(fh);
    let ret_v: T = serde_json::from_reader(bufr)?;
    Ok(ret_v)
}

pub fn to_file<T: serde::Serialize>(
    file_path: &str,
    truncate: bool,
    data: T,
) -> Result<(), Box<dyn Error>> {
    let fh = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(truncate)
        .open(file_path)?;
    let bufw = BufWriter::new(fh);
    Ok(serde_json::to_writer(bufw, &data)?)
}
