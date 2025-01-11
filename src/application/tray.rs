use ksni::Tray;

use crate::capture::capture_and_upload;
use crate::application::ui::settings::spawn_settings_window;

#[derive(Debug, Default)]
pub(crate) struct ShareShotTray;

impl Tray for ShareShotTray {
    fn id(&self) -> String {
        "shareshot".into()
    }

    fn icon_name(&self) -> String {
        "help-about".into()
    }

    fn title(&self) -> String {
        "ShareShot".into()
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: "Capture".into(),
                activate: Box::new(|_| {
                    tokio::spawn(async move {
                        capture_and_upload().await.unwrap();
                    });
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Settings".into(),
                activate: Box::new(|_| {
                   tokio::spawn(async move {
                        spawn_settings_window().await;
                   });
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}
