use reqwest::{
    multipart::{Form, Part},
    RequestBuilder,
};

use crate::{
    capture::ImageData,
    config::{UploadConfig, UploadStrategy},
};

use super::CLIENT;

pub struct UploadRequestBuilder<'a> {
    config: &'a UploadConfig,
    image: Option<ImageData>,
}

impl<'a> UploadRequestBuilder<'a> {
    pub fn new(config: &'a UploadConfig) -> Self {
        Self {
            config,
            image: None,
        }
    }

    pub fn with_image(mut self, image: ImageData) -> Self {
        self.image = Some(image);
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut builder = CLIENT.request((&self.config.request_method).into(), &self.config.url);

        for (key, value) in &self.config.headers {
            builder = builder.header(key, value);
        }

        if let Some(image) = self.image {
            builder = match self.config.upload_strategy {
                UploadStrategy::Body => builder.body(image.bytes),
                UploadStrategy::Multipart => {
                    builder.multipart(
                        Form::new().part(
                            self.config.file_form_name.clone().unwrap_or_default(),
                            Part::bytes(image.bytes)
                                // TODO: add support for file names
                                .file_name("temp")
                                .mime_str(&image.mime_type)
                                .unwrap(),
                        ),
                    )
                }
            };
        }

        builder
    }
}
