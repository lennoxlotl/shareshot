use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use crate::error::Error;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// The different strategies which can be used to upload the image data.
#[derive(Debug, Serialize, Deserialize)]
pub enum UploadStrategy {
    Body,
    Multipart,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
}

/// The configuration for the upload server.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UploadConfig {
    pub url: String,
    pub request_method: RequestMethod,
    pub headers: HashMap<String, String>,
    pub upload_strategy: UploadStrategy,
    pub file_form_name: Option<String>,
    pub url_parser: String,
}

/// The main configuration file
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShareShotConfig {
    // Deletes a screenshot after it was read
    pub cleanup: bool,
    pub upload_server: UploadConfig,
}

impl Default for UploadStrategy {
    fn default() -> Self {
        Self::Multipart
    }
}

impl Default for RequestMethod {
    fn default() -> Self {
        Self::Put
    }
}

impl Into<Method> for &RequestMethod {
    fn into(self) -> Method {
        match self {
            RequestMethod::Get => Method::GET,
            RequestMethod::Put => Method::PUT,
            RequestMethod::Post => Method::POST,
        }
    }
}

/// Loads the config from the file system or with default values if it doesn't exist.
pub fn load_config() -> Result<ShareShotConfig, Error> {
    match config_path() {
        Some(path) => {
            if path.exists() {
                let mut file = File::open(path).map_err(|err| Error::from(err))?;
                let mut content = String::new();

                file.read_to_string(&mut content)
                    .map_err(|err| Error::from(err))?;
                return toml::from_str::<ShareShotConfig>(content.as_str())
                    .map_err(|_| Error::ConfigLoad);
            }

            let config_json = toml::to_string_pretty(&ShareShotConfig::default())
                .map_err(|_| Error::ConfigLoad)?;
            fs::write(path, config_json).map_err(|err| Error::from(err))?;

            Ok(ShareShotConfig::default())
        }
        None => Ok(ShareShotConfig::default()),
    }
}

/// Creates and returns the config path.
fn config_path() -> Option<PathBuf> {
    let mut home_dir = home::home_dir()?;
    home_dir.push(".config");
    home_dir.push("shareshot");
    fs::create_dir_all(&home_dir).ok()?;
    home_dir.push("config.toml");
    Some(home_dir)
}
