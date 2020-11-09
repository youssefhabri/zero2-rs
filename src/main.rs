#![feature(async_closure)]

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
    kankyo::load(false).expect("Failed to load .env file");
    env_logger::init();

    let mut client = Zero2Client::new().await;

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
