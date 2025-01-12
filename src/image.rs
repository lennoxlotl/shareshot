use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::error::Error;

/// Stores data about a read image file.
pub struct Image {
    path: PathBuf,
    bytes: Vec<u8>,
    mime_type: String,
}

impl Image {
    /// Reads an image from a given path.
    ///
    /// # Returns
    /// The image data
    pub fn read<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path> + ToString + Display,
    {
        let path_ref = path.as_ref();

        if !path_ref.exists() {
            return Err(Error::ImageNotFound);
        }

        let mime_type = mime_guess::from_path(&path);
        let mut file = File::open(path_ref).map_err(|err| Error::from(err))?;
        let mut image_bytes = Vec::new();
        file.read_to_end(&mut image_bytes)
            .map_err(|err| Error::from(err))?;

        Ok(Self {
            path: path.as_ref().to_path_buf(),
            bytes: image_bytes,
            mime_type: mime_type.first_or_octet_stream().to_string(),
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }
}
