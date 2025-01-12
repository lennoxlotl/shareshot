use std::{collections::BTreeMap, str};

use crate::{
    application::CONFIG,
    config::{AllEnumValues, RequestMethod, UploadStrategy},
};
use adw::prelude::*;
use enum_ordinalize::Ordinalize;
use relm4::{abstractions::Toaster, prelude::*, AsyncComponentSender};

use super::{factory::header::{VisualizedHeader, VisualizedHeaderMessage}, save_with_report};

pub struct UploadPage {
    current_url: String,
    current_file_form_name: String,
    current_url_parser: String,
    selected_request_method: i8,
    selected_upload_strategy: i8,
    headers: AsyncFactoryVecDeque<VisualizedHeader>,
    toaster: Toaster,
}

#[derive(Debug)]
pub enum UploadPageMessage {
    AddHeader,
    RemoveHeader(DynamicIndex),
    ChangeHeader,
    ChangeUploadStrategy(u32),
    ChangeRequestMethod(u32),
    ChangeUrl(String),
    ChangeFileFormName(String),
    ChangeUrlParser(String),
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for UploadPage {
    type Init = ();
    type Input = UploadPageMessage;
    type Output = ();

    view! {
        gtk4::Box {
            set_orientation: gtk4::Orientation::Vertical,

            #[local_ref]
            toast_overlay -> adw::ToastOverlay {
                set_vexpand: true,
                adw::PreferencesPage {
                    adw::PreferencesGroup {
                        set_title: "General",
                        gtk4::ListBox {
                            add_css_class: "boxed-list",
                            set_selection_mode: gtk4::SelectionMode::None,

                            adw::EntryRow {
                                set_title: "URL",
                                set_text: &model.current_url,
                                connect_changed[sender] => move |entry| {
                                    sender.input(UploadPageMessage::ChangeUrl(entry.text().to_string()));
                                }
                            },
                            adw::ComboRow {
                                set_title_lines: 1,
                                set_subtitle_lines: 1,
                                set_title: "Request Method",
                                set_subtitle: "The REST method to use when making the upload request",
                                set_model: Some(&UploadPage::extract_strings_from::<RequestMethod>()),
                                set_selected: model.selected_request_method as u32,
                                connect_selected_notify[sender] => move |item| {
                                sender.input(UploadPageMessage::ChangeRequestMethod(item.selected()));
                                },
                            },
                            adw::ComboRow {
                                set_title_lines: 1,
                                set_subtitle_lines: 1,
                                set_title: "Upload Strategy",
                                set_subtitle: "The method to use for attaching the image to the REST request",
                                set_model: Some(&UploadPage::extract_strings_from::<UploadStrategy>()),
                                set_selected: model.selected_upload_strategy as u32,
                                connect_selected_notify[sender] => move |item| {
                                sender.input(UploadPageMessage::ChangeUploadStrategy(item.selected()));
                                },
                            },
                            #[name(multipart_file_name)]
                            adw::EntryRow {
                                set_title: "Multipart File Name",
                                set_text: &model.current_file_form_name,
                                #[watch]
                                set_visible: model.selected_upload_strategy == UploadStrategy::Multipart.ordinal() as i8,
                                connect_changed[sender] => move |entry| {
                                    sender.input(UploadPageMessage::ChangeFileFormName(entry.text().to_string()));
                                }
                            },
                            adw::EntryRow {
                                set_title: "Response Parse Pattern",
                                set_tooltip_text: Some("Parser Options:\n* $raw$ - Copies the raw response content into clipboard\n* $json:data.key$ Copies the JSON value at `data.key` into clipboard"),
                                set_text: &model.current_url_parser,
                                connect_changed[sender] => move |entry| {
                                    sender.input(UploadPageMessage::ChangeUrlParser(entry.text().to_string()));
                                }
                            },
                        }
                    },
                    adw::PreferencesGroup {
                        set_title: "Headers",
                        #[wrap(Some)]
                        set_header_suffix = &gtk4::Box {
                            add_css_class: "linked",

                            gtk4::Button {
                                set_css_classes: &vec!["flat"],
                                set_icon_name: "plus",
                                connect_clicked: move |_| {
                                    sender.input(UploadPageMessage::AddHeader);
                                }
                            }
                        },

                        #[local_ref]
                        header_box -> gtk4::Box {
                            set_orientation: gtk4::Orientation::Vertical,
                            set_spacing: 4,
                        },
                    }
                },
            }
        }
    }

    async fn init(
        _app: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let config = CONFIG.lock().await;
        let mut headers = AsyncFactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                VisualizedHeaderMessage::Change => UploadPageMessage::ChangeHeader,
                VisualizedHeaderMessage::Delete(index) => UploadPageMessage::RemoveHeader(index),
            });

        for (key, value) in &config.upload_server.headers {
            headers.guard().push_back((key.clone(), value.clone()));
        }

        let model = Self {
            current_url: config.upload_server.url.clone(),
            current_file_form_name: config
                .upload_server
                .file_form_name
                .clone()
                .unwrap_or_default(),
            current_url_parser: config.upload_server.url_parser.clone(),
            selected_request_method: config.upload_server.request_method.ordinal(),
            selected_upload_strategy: config.upload_server.upload_strategy.ordinal(),
            headers,
            toaster: Toaster::default(),
        };

        let toast_overlay = model.toaster.overlay_widget();
        let header_box = model.headers.widget();
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {
        match message {
            UploadPageMessage::AddHeader => {
                self.headers
                    .guard()
                    .push_back((String::new(), String::new()));

                self.save_with_headers().await;
            }
            UploadPageMessage::RemoveHeader(key) => {
                self.headers.guard().remove(key.current_index());

                self.save_with_headers().await;
            }
            UploadPageMessage::ChangeHeader => {
                self.save_with_headers().await;
            }
            UploadPageMessage::ChangeUploadStrategy(index) => {
                self.selected_upload_strategy = index as i8;
                self.save_without_headers().await;
            }
            UploadPageMessage::ChangeRequestMethod(index) => {
                self.selected_request_method = index as i8;
                self.save_without_headers().await;
            }
            UploadPageMessage::ChangeUrl(url) => {
                self.current_url = url.clone();
                self.save_without_headers().await;
            }
            UploadPageMessage::ChangeFileFormName(file_form_name) => {
                self.current_file_form_name = file_form_name.clone();
                self.save_without_headers().await;
            }
            UploadPageMessage::ChangeUrlParser(url_parser) => {
                self.current_url_parser = url_parser.clone();
                self.save_without_headers().await;
            }
        }
    }
}

impl UploadPage {
    async fn save_without_headers(&mut self) {
        let mut config = CONFIG.lock().await;

        config.upload_server.set_url(self.current_url.clone());
        config
            .upload_server
            .set_file_form_name(self.current_file_form_name.clone());
        config
            .upload_server
            .set_url_parser(self.current_url_parser.clone());
        config.upload_server.set_request_method(
            RequestMethod::from_ordinal(self.selected_request_method).unwrap_or_default(),
        );
        config.upload_server.set_upload_strategy(
            UploadStrategy::from_ordinal(self.selected_upload_strategy).unwrap_or_default(),
        );

        save_with_report(&config, &self.toaster).await;
    }

    async fn save_with_headers(&mut self) {
        let mut config = CONFIG.lock().await;

        let mut new_headers = BTreeMap::new();
        self.headers
            .iter()
            .filter_map(|header| header)
            .for_each(|header| {
                new_headers.insert(header.key.clone(), header.value.clone());
            });
        config.upload_server.headers = new_headers;

        save_with_report(&config, &self.toaster).await;
    }

    fn extract_strings_from<T>() -> gtk4::StringList
    where
        T: AllEnumValues + Copy,
        &'static str: From<T>,
    {
        gtk4::StringList::new(&T::all().iter().map(|v| (*v).into()).collect::<Vec<&str>>())
    }
}
