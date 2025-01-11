use reqwest::{
    multipart::{Form, Part},
    RequestBuilder,
};

use crate::{
    config::{UploadConfig, UploadStrategy},
    image::Image,
};

use super::CLIENT;

/// Used to build a request for uploading an image to the upload server defined in the
/// configuration file.
///
/// # Example
///
/// ```rust
/// let response =
///     RequestBuilder::from(ImageUploadRequest::new(&some_config, some_image))
///         .send()
///         .await
///         .unwrap();
/// println!("body: {:#?}", response.text().await.unwrap());
/// ```
pub struct ImageUploadRequest<'a> {
    config: &'a UploadConfig,
    image: Image,
}

impl<'a> ImageUploadRequest<'a> {
    pub fn new(config: &'a UploadConfig, image: Image) -> Self {
        Self { config, image }
    }
}

impl<'a> From<ImageUploadRequest<'a>> for RequestBuilder {
    fn from(upload: ImageUploadRequest) -> RequestBuilder {
        let mut builder =
            CLIENT.request((&upload.config.request_method).into(), &upload.config.url);

        for (key, value) in &upload.config.headers {
            builder = builder.header(key, value);
        }

        // Yes, heavy operation but it seems like it cannot be avoided due to 'static requirements
        let cloned_bytes = upload.image.bytes().clone();
        builder = match upload.config.upload_strategy {
            UploadStrategy::Body => builder.body(cloned_bytes),
            UploadStrategy::Multipart => {
                builder.multipart(
                    Form::new().part(
                        upload.config.file_form_name.clone().unwrap_or_default(),
                        Part::bytes(cloned_bytes)
                            // TODO: add support for file names
                            .file_name("temp")
                            .mime_str(&upload.image.mime_type())
                            .unwrap(),
                    ),
                )
            }
        };

        builder
    }
}
