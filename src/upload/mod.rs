use once_cell::sync::Lazy;
use reqwest::{Client, RequestBuilder};

use crate::{application::CONFIG, error::Error, image::Image, parser::parse_url};

use self::request::ImageUploadRequest;

pub mod request;

pub(crate) static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

/// Uploads an image to the in the config-defined upload server.
///
/// # Returns
/// The url to the uploaded image
pub async fn upload_image(image: &Image) -> Result<String, Error> {
    let config = &CONFIG.lock().await.upload_server;
    let response = RequestBuilder::from(ImageUploadRequest::new(&config, image))
        .send()
        .await
        .map_err(|err| Error::from(err))?;

    let status = response.status();
    let text = response.text().await.map_err(|err| Error::from(err))?;
    if !status.is_success() {
        return Err(Error::NonOkStatusCode(status.to_string(), text));
    }

    Ok(parse_url(&text, &config.url_parser)?)
}
