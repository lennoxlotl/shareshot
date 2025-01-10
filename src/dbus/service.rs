use zbus::{connection, interface, Connection};

use crate::error::Error;

#[derive(Default)]
pub struct CaptureService;

#[interface(name = "dev.lennoxlotl.ShareShot.CaptureService")]
impl CaptureService {
    fn request_capture(&mut self, region: bool, display: &str) -> String {
        "Hello".to_string()
    }
}

pub async fn create_dbus_service() -> Result<Connection, Error> {
    let service = CaptureService::default();
    connection::Builder::session()
        .map_err(|err| Error::DbusCreate(Box::new(err)))?
        .name("dev.lennoxlotl.ShareShot")
        .map_err(|err| Error::DbusCreate(Box::new(err)))?
        .serve_at("/dev/lennoxlotl/ShareShot/CaptureService", service)
        .map_err(|err| Error::DbusCreate(Box::new(err)))?
        .build()
        .await
        .map_err(|err| Error::from(err))
}
