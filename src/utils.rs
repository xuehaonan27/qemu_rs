use std::{error::Error, fs::File, io::Read, path::Path};

use crate::configuration::*;

use self::general::config;

pub fn read_json_from_file<'a, P: AsRef<Path>>(
    path: P,
    string: &'a mut String,
) -> Result<config::Config<'a>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    file.read_to_string(string)?;
    let c: config::Config = serde_json::from_str(string)?;
    Ok(c)
}
