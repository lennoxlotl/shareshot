use zbus::{connection, interface, Connection};

use crate::{capture::capture_and_upload, error::Error};

#[derive(Default)]
pub struct CaptureService;

#[interface(name = "dev.lennoxlotl.ShareShot.CaptureService")]
impl CaptureService {
    async fn request_capture(&mut self) -> String {
        match capture_and_upload().await {
            Ok(_) => "Upload successful".into(),
            Err(err) => format!("Failed to upload: {}", err),
        }
    }
}

pub async fn create_dbus_service() -> Result<Connection, Error> {
    let service = CaptureService::default();
    connection::Builder::session()?
        .name("dev.lennoxlotl.ShareShot")?
        .serve_at("/dev/lennoxlotl/ShareShot/CaptureService", service)?
        .build()
        .await
        .map_err(|err| Error::from(err))
}
