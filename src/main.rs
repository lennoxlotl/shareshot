use clap::Parser;
use log::error;

pub mod application;
pub mod capture;
pub mod config;
pub mod dbus;
pub mod error;
pub mod image;
pub mod parser;
pub mod upload;

#[derive(Parser, Debug)]
struct ShareShotArgs {
    #[arg(long, default_value_t = false)]
    capture: bool,
}

impl ShareShotArgs {
    fn capture(&self) -> bool {
        self.capture
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = ShareShotArgs::parse();

    if args.capture() {
    } else {
        match application::create_application().await {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to launch ShareShot ({})", err);
            }
        }
    }
}
