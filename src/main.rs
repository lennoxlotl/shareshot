use clap::Parser;
use log::error;

pub mod application;
pub mod dbus;
pub mod error;
pub mod capture;
pub mod upload;
pub mod config;
pub mod parser;
pub mod image;

#[derive(Parser, Debug)]
struct ShareShotArgs {
    #[arg(long, default_value_t = false)]
    capture_region: bool,
    #[arg(long)]
    capture_display: Option<String>,
}

impl ShareShotArgs {
    fn should_request_capture(&self) -> bool {
        self.capture_region || self.capture_display.is_some()
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = ShareShotArgs::parse();

    if args.should_request_capture() {
    } else {
        match application::create_application().await {
            Ok(_) => {
            },
            Err(err) => {
                error!("Failed to launch ShareShot ({})", err);
            },
        }
    }
}
