// #![feature(async_closure)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod client;
mod commands;
mod core;
#[macro_use]
mod macros;
mod monitors;
mod utils;

use crate::client::Zero2Client;

#[tokio::main]
async fn main() {
    if let Err(_) = kankyo::load(false) {
        warn!("Failed to load .env file. Falling back to env variables.")
    }

    if let Err(why) = setup_logger() {
        warn!("Failed to initialize logger: {}", why);
    }

    let mut client = Zero2Client::new().await;

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Error)
        .chain(std::io::stdout())
        .chain(fern::log_file("error_logs.log")?)
        .apply()?;

    Ok(())
}
