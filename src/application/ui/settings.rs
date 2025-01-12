use gtk4::prelude::*;
use relm4::prelude::*;

struct SettingsApp {}

#[derive(Debug)]
enum SettingsMessage {}

#[relm4::component(async)]
impl SimpleAsyncComponent for SettingsApp {
    type Init = u8;
    type Input = SettingsMessage;
    type Output = ();

    view! {
        gtk4::Window {
        }
    }

    async fn init(
        app: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {}
}

pub async fn spawn_settings_window() {
    let app = RelmApp::new("dev.lennoxlotl.ShareShot");
    app.run_async::<SettingsApp>(0);
}
