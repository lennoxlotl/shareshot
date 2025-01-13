use std::process::Command;

use ashpd::desktop::screenshot::Screenshot;

use crate::{error::Error, image::Image, upload::upload_image};

/// Makes a screen capture and uploads it to the server defined in the configuration.
pub async fn capture_and_upload() -> Result<(), Error> {
    let image = make_screen_capture().await?;
    let url = upload_image(image).await?;

    // TODO: Exchange this with proper clipboard copy if a protocol ever gets added
    Command::new("wl-copy")
        .arg(url)
        .spawn()
        .map_err(|err| Error::from(err))?;

    Ok(())
}

/// Requests the screen capture using xdg-desktop-portal.
///
/// # Returns
/// The path to the screenshot file
pub async fn make_screen_capture() -> Result<Image, Error> {
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
        .map_err(|err| Error::from(err))?
        .to_string();
    Image::read(path)
}
