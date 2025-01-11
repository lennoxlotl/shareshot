use ksni::TrayMethods;
use log::{info};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    config::{load_config, ShareShotConfig},
    error::Error,
};

use self::tray::ShareShotTray;

pub(crate) mod tray;
pub(crate) mod ui;

pub static CONFIG: Lazy<Arc<Mutex<ShareShotConfig>>> =
    Lazy::new(|| Arc::new(Mutex::new(load_config().unwrap_or_default())));

pub async fn create_application() -> Result<(), Error> {
    let _conn = crate::dbus::service::create_dbus_service().await?;
    info!("Created DBus service successfully");

    let tray = ShareShotTray::default();
    tray.spawn().await.unwrap();
    info!("Created tray icon successfully");

    std::future::pending::<()>().await;
    Ok(())
}
