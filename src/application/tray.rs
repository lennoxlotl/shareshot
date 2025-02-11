use ksni::Tray;

use crate::{application::ApplicationMessage, capture::capture_and_upload};

pub(crate) struct ShareShotTray {
    pub sender: relm4::Sender<ApplicationMessage>,
}

// TODO: Replace this with `Background Apps` actions once the feature is available in GNOME (keep this as fallback?)
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
                        capture_and_upload()
                            .await
                            .expect("Failed to capture, screenshot has been canceled?");
                    });
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Settings".into(),
                activate: Box::new(|tray: &mut Self| {
                    let _ = tray.sender.send(ApplicationMessage::ShowSettingsWindow);
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
