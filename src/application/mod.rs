use gtk4::prelude::{GtkApplicationExt, GtkWindowExt, WidgetExt};
use ksni::TrayMethods;
use log::info;
use once_cell::sync::Lazy;
use relm4::{
    prelude::{AsyncComponentParts, SimpleAsyncComponent},
    AsyncComponentSender, RelmApp,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    config::{load_config, ShareShotConfig},
    error::Error,
};

use self::tray::ShareShotTray;

pub(crate) mod tray;

pub static CONFIG: Lazy<Arc<Mutex<ShareShotConfig>>> =
    Lazy::new(|| Arc::new(Mutex::new(load_config().unwrap_or_default())));

pub struct Application {}

#[derive(Debug)]
pub enum ApplicationMessage {
    ShowSettingsWindow,
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for Application {
    type Init = u8;
    type Input = ApplicationMessage;
    type Output = ();

    view! {
        gtk4::Window {}
    }

    async fn init(
        _app: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        root.set_hide_on_close(true);

        let tray = ShareShotTray {
            sender: sender.input_sender().clone(),
        };

        tray.spawn().await.unwrap();

        let model = Self {};
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {
        match message {
            ApplicationMessage::ShowSettingsWindow => {
                let app = relm4::main_adw_application();
                let windows = app.windows();
                let window = windows.first().unwrap();
                window.set_visible(true);
            }
        }
    }
}

pub async fn create_application() -> Result<(), Error> {
    let _conn = crate::dbus::service::create_dbus_service().await?;
    info!("Created DBus service successfully");

    RelmApp::new("dev.lennoxlotl.ShareShotWindow")
        .visible_on_activate(false)
        .run_async::<Application>(0);

    //  let tray = ShareShotTray::default();
    //    tray.spawn().await.unwrap();
    //info!("Created tray icon successfully");

    Ok(())
}
