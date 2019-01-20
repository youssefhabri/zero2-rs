#![feature(inner_deref)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rust_embed;

use std::collections::{HashSet, HashMap};
use std::sync::Arc;
use serenity::{
    client::Client,
    framework::standard::StandardFramework,
    http
};

mod structs;
mod commands;

use crate::structs::{CommandCounter, Handler, ShardManagerContainer};


fn main() {
    // Load token from environment variables or .env file
    let token: String = dotenv::var("DISCORD_TOKEN").expect("token");
    let prefix: String = dotenv::var("BOT_PREFIX").expect("prefix");

    // Initialize the logger
    //env_logger::init();
    pretty_env_logger::init();

    let mut client = Client::new(
        &token,
        Handler
    ).expect("Error creating client");

    {
        let mut data = client.data.lock();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .prefix(&prefix)
            .owners(owners))
        .command("ping", |c| c.cmd(commands::meta::Ping))
        .command("anime", |c| c.cmd(commands::anilist::AnimeCommand))
        .command("test", |c| c.cmd(commands::meta::Test))
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
