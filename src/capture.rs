use arboard::Clipboard;
use ashpd::desktop::screenshot::Screenshot;

use crate::{application::CONFIG, error::Error, image::Image, upload::upload_image};

/// Makes a screen capture and uploads it to the server defined in the configuration.
pub async fn capture_and_upload() -> Result<(), Error> {
    let image = make_screen_capture().await?;
    let url = upload_image(&image).await?;

    Clipboard::new()?.set_text(url)?;

    let config = CONFIG.lock().await;
    if config.cleanup {
        std::fs::remove_file(image.path()).map_err(|err| Error::from(err))?;
    }

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
