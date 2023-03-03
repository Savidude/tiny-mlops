use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io::copy;

use serde::de::DeserializeOwned;
use serde::{Serialize};

use reqwest;

use crate::results::FileCreateResult;

#[tokio::main]
pub async fn download_from_url(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let mut file = File::create(destination)?;
    copy(&mut bytes.as_ref(), &mut file)?;
    Ok(())
}

pub fn create_dir_if_exists(filepath: &str)  -> FileCreateResult<> {
    if Path::new(filepath).is_dir() {
        FileCreateResult::FileExists()
    } else {
        match create_dir(filepath) {
            Ok(()) => {    
                FileCreateResult::OK()
            },
            Err(_) => {
                FileCreateResult::Error(String::from("Error while creating directory"))
            },
        }
    }
}

pub fn file_exists(filepath: &str) -> bool {
    Path::new(filepath).exists()
}

pub fn read_json<T: DeserializeOwned>(filename: &str) -> Result<T, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;

    Ok(data)
}

pub fn write_to_json<T: Serialize>(data: T, filename: &str) -> std::io::Result<()> {
    let json = serde_json::to_string(&data)?;

    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

fn create_dir(filepath: &str) -> std::io::Result<()> {
    fs::create_dir_all(filepath)?;
    Ok(())
}
