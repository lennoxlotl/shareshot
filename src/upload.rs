use once_cell::sync::Lazy;
use reqwest::{
    multipart::{Form, Part},
    Client,
};

use crate::{
    application::CONFIG, capture::ImageData, config::UploadStrategy, error::Error,
    parser::parse_response,
};

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

/// Uploads an image to the in the config-defined upload server.
///
/// # Returns
/// The url to the uploaded image
pub async fn upload_image(image: ImageData) -> Result<String, Error> {
    let config = &CONFIG.lock().await.upload_server;
    let mut request = CLIENT.request((&config.request_method).into(), &config.url);

    for header in &config.headers {
        request = request.header(header.0, header.1);
    }
    match &config.upload_strategy {
        UploadStrategy::Body => request = request.body(image.bytes),
        UploadStrategy::Multipart => {
            request = request.multipart(
                Form::new().part(
                    config.file_form_name.clone().unwrap_or_default(),
                    Part::bytes(image.bytes)
                        // TODO: add support for file names
                        .file_name("temp")
                        .mime_str(&image.mime_type)
                        .unwrap(),
                ),
            )
        }
    };

    let response = request.send().await.map_err(|err| Error::from(err))?;
    if !response.status().is_success() {
        return Err(Error::NonOkStatusCode(
            response.status().to_string(),
            response.text().await.map_err(|err| Error::from(err))?,
        ));
    }

    Ok(parse_response(
        &response.text().await.map_err(|err| Error::from(err))?,
        &config.url_parser,
    )?)
}
