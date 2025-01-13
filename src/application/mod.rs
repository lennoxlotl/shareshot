use adw::prelude::*;
use general::GeneralPage;
use gtk4::prelude::{GtkApplicationExt, GtkWindowExt, WidgetExt};
use ksni::TrayMethods;
use log::info;
use once_cell::sync::Lazy;
use relm4::{
    abstractions::Toaster,
    actions::{ActionGroupName, RelmAction, RelmActionGroup},
    component::AsyncConnector,
    prelude::*,
    AsyncComponentSender, RelmApp,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use upload::UploadPage;

use crate::{
    config::{load_config, ShareShotConfig},
    error::Error,
};

use self::tray::ShareShotTray;

pub(crate) mod factory;
pub(crate) mod general;
pub(crate) mod tray;
pub(crate) mod upload;
pub(crate) mod icon_names {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

pub static CONFIG: Lazy<Arc<Mutex<ShareShotConfig>>> =
    Lazy::new(|| Arc::new(Mutex::new(load_config().unwrap_or_default())));

pub struct Application {
    general_page: AsyncConnector<GeneralPage>,
    upload_page: AsyncConnector<UploadPage>,
}

#[derive(Debug)]
pub enum ApplicationMessage {
    ShowSettingsWindow,
}

relm4::new_action_group!(ApplicationActionGroup, "group");
relm4::new_stateless_action!(ShowAboutDialog, ApplicationActionGroup, "show-about-dialog");

#[relm4::component(pub async)]
impl SimpleAsyncComponent for Application {
    type Init = u8;
    type Input = ApplicationMessage;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_size_request: (660, 600),

            adw::ToolbarView {
                #[wrap(Some)]
                set_content = &gtk4::Box {
                    set_orientation: gtk4::Orientation::Vertical,

                    adw::Clamp {
                        #[name = "view_stack"]
                        adw::ViewStack {
                            set_vexpand: true,
                            add_titled_with_icon: (model.general_page.widget(), Some("general"), "General", crate::application::icon_names::SETTINGS),
                            add_titled_with_icon: (model.upload_page.widget(), Some("upload"), "Upload", crate::application::icon_names::SHARE),
                        },
                    }
                },
                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    #[name = "view_switcher"]
                    set_title_widget = &adw::ViewSwitcher {
                        set_policy: adw::ViewSwitcherPolicy::Wide,
                        set_stack = Some(&view_stack),
                    },
                    pack_end = &gtk4::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&window_menu),
                    }
                },
            }
        }
    }

    menu! {
        window_menu: {
            section! {
                "About" => ShowAboutDialog,
            }
        }
    }

    async fn init(
        _app: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let mut group = RelmActionGroup::<ApplicationActionGroup>::new();
        let cloned_root = root.clone();

        // TODO: move this into a component
        group.add_action(RelmAction::<ShowAboutDialog>::new_stateless(move |_| {
            let dialog = adw::AboutDialog::builder()
                .application_name("ShareShot")
                .developer_name("Lennox Schneider")
                .version("1.0.0")
                .comments("The best screenshot uploader app.")
                .website("https://github.com/lennoxlotl/shareshot")
                .license_type(gtk4::License::MitX11)
                .build();

            dialog.present(Some(&cloned_root));
        }));

        root.set_hide_on_close(true);
        root.insert_action_group(
            ApplicationActionGroup::NAME,
            Some(&group.into_action_group()),
        );

        let tray = ShareShotTray {
            sender: sender.input_sender().clone(),
        };

        tray.spawn().await.unwrap();

        let model = Self {
            general_page: GeneralPage::builder().launch(()),
            upload_page: UploadPage::builder().launch(()),
        };
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

pub(crate) async fn save_with_report(config: &ShareShotConfig, toaster: &Toaster) {
    match config.save() {
        Ok(_) => {}
        Err(err) => {
            toaster.add_toast(
                adw::Toast::builder()
                    .title("Failed to save config")
                    .timeout(10000)
                    .build(),
            );
            log::error!("Failed to save config: {}", err)
        }
    };
}

pub async fn create_application() -> Result<(), Error> {
    let _conn = crate::dbus::service::create_dbus_service().await?;
    info!("Created DBus service successfully");

    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);
    RelmApp::new("dev.lennoxlotl.ShareShotSettings")
        .visible_on_activate(false)
        .run_async::<Application>(0);

    Ok(())
}
