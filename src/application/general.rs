use crate::{application::CONFIG, config::save_config};
use adw::prelude::*;
use relm4::{prelude::*, AsyncComponentSender};

pub struct GeneralPage {
    cleanup_state: bool,
}

#[derive(Debug)]
pub enum GeneralPageMessage {
    SetCleanup(bool),
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for GeneralPage {
    type Init = ();
    type Input = GeneralPageMessage;
    type Output = ();

    view! {
        gtk4::Box {
            set_orientation: gtk4::Orientation::Vertical,

            adw::PreferencesPage {
                adw::PreferencesGroup {
                    set_title: "Image",
                    gtk4::ListBox {
                        add_css_class: "boxed-list",
                        set_selection_mode: gtk4::SelectionMode::None,

                        #[name = "cleanup"]
                        adw::SwitchRow {
                            set_title: "Cleanup",
                            set_subtitle: "Deletes the screenshot file after upload",
                            set_active: model.cleanup_state,
                            connect_active_notify[sender] => move |switch| {
                                sender.input(GeneralPageMessage::SetCleanup(switch.is_active()))
                            },
                        }
                    }
                }
            },
        }
    }

    async fn init(
        _app: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let config = CONFIG.lock().await;

        let model = Self {
            cleanup_state: config.cleanup,
        };
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {
        match message {
            GeneralPageMessage::SetCleanup(active) => {
                let mut config = CONFIG.lock().await;

                self.cleanup_state = active;
                config.cleanup = active;

                save_config(&config).unwrap();
            }
        }
    }
}
