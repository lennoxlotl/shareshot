use adw::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub struct VisualizedHeader {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub enum VisualizedHeaderMessage {
    Change,
    Delete(DynamicIndex),
}

#[derive(Debug)]
pub enum VisualizedHeaderInput {
    ChangeKey(String),
    ChangeValue(String),
}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for VisualizedHeader {
    type Init = (String, String);
    type Input = VisualizedHeaderInput;
    type Output = VisualizedHeaderMessage;
    type CommandOutput = ();
    type ParentWidget = gtk4::Box;

    view! {
        gtk4::Box {
            set_orientation: gtk4::Orientation::Horizontal,
            set_spacing: 6,

            gtk4::Box {
                set_orientation: gtk4::Orientation::Horizontal,
                add_css_class: "linked",

                gtk4::Entry {
                    set_hexpand: true,
                    set_placeholder_text: Some("Key"),
                    set_editable: true,
                    set_text: &self.key,
                    connect_changed[sender] => move |entry| {
                        sender.input(VisualizedHeaderInput::ChangeKey(entry.text().to_string()));
                    }
                },
                gtk4::Entry {
                    set_hexpand: true,
                    set_placeholder_text: Some("Value"),
                    set_editable: true,
                    set_text: &self.value,
                    connect_changed[sender] => move |entry| {
                        sender.input(VisualizedHeaderInput::ChangeValue(entry.text().to_string()));
                    }
                },
            },
            gtk4::Button {
                set_css_classes: &vec!["flat"],
                set_icon_name: crate::application::icon_names::CROSS_LARGE,
                connect_clicked[sender, index] => move |_| {
                    match sender.output(VisualizedHeaderMessage::Delete(index.clone())) {
                        Ok(_) => {},
                        Err(_) => log::error!("Failed to send Header delete to parent widget"),
                    };
                }
            }
        }
    }

    async fn init_model(
        value: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        Self {
            key: value.0,
            value: value.1,
        }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncFactorySender<Self>) {
        match msg {
            VisualizedHeaderInput::ChangeKey(key) => {
                self.key = key;
            }
            VisualizedHeaderInput::ChangeValue(value) => {
                self.value = value;
            }
        }

        match sender.output(VisualizedHeaderMessage::Change) {
            Ok(_) => {},
            Err(_) => log::error!("Failed to send Header changed to parent"),
        }
    }
}
