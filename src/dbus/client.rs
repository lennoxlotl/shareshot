use zbus::{proxy, Connection};

use crate::error::Error;

#[proxy(
    interface = "dev.lennoxlotl.ShareShot.CaptureService",
    default_service = "dev.lennoxlotl.ShareShot",
    default_path = "/dev/lennoxlotl/ShareShot/CaptureService"
)]
pub trait CaptureService {
    async fn request_capture(&self) -> zbus::Result<String>;
}

/// Requests a capture by invoking the dbus service.
pub async fn request_capture() -> Result<(), Error> {
    let connection = Connection::session().await?;
    let proxy = CaptureServiceProxy::new(&connection).await?;
    let reply = proxy.request_capture().await?;
    log::info!("dbus daemon returned: {reply}");
    Ok(())
}
