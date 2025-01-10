use ashpd::desktop::screenshot::Screenshot;
use log::debug;
use std::{fs::File, io::Read};

use crate::{error::Error, upload::upload_image};

#[derive(Debug)]
pub struct ImageData {
    pub bytes: Vec<u8>,
    pub mime_type: String,
}

impl ImageData {
    pub fn new(bytes: Vec<u8>, mime_type: String) -> Self {
        Self { bytes, mime_type }
    }
}

/// Makes a screen capture and uploads it to the server defined in the configuration.
pub async fn capture_and_upload() -> Result<(), Error> {
    let image = make_screen_capture(true).await?;
    println!("{}", upload_image(image).await?);
    Ok(())
}

/// Requests the screen capture using xdg-desktop-portal.
///
/// # Args
/// * `cleanup` - Will delete the file after read
///
/// # Returns
/// The path to the screenshot file
pub async fn make_screen_capture(cleanup: bool) -> Result<ImageData, Error> {
    let mut path: String = Screenshot::request()
        .interactive(true)
        .send()
        .await
        .map_err(|err| Error::from(err))?
        .response()
        .map_err(|err| Error::from(err))?
        .uri()
        .path()
        .into();
    path = urlencoding::decode(path.as_str())
        .map_err(|_| Error::UriCodec)?
        .to_string();

    debug!("reading screenshot file: {}", &path);
    let mime_type = mime_guess::from_path(&path);
    debug!("detected mime type {:#?} for {}", &mime_type, &path);
    let mut file = File::open(&path).map_err(|err| Error::from(err))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|_| Error::InvalidScreenshotFile)?;

    if cleanup {
        debug!("deleting screenshot file: {}", &path);
        std::fs::remove_file(path).map_err(|err| Error::from(err))?;
    }

    Ok(ImageData::new(
        bytes,
        mime_type.first_or_octet_stream().to_string(),
    ))
}
