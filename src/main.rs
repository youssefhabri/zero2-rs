//#![feature(inner_deref)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rust_embed;

use std::collections::{HashSet, HashMap};
use std::sync::Arc;
use serenity::{
    client::Client,
    framework::standard::StandardFramework,
    http,
};

mod commands;
mod handler;
mod menu;
mod models;
mod monitors;
mod store;
mod utils;

use crate::store::{
    BotOwnerContainer,
    CommandCounter,
    MessagePaginator,
    ShardManagerContainer,
};
use crate::handler::Zero2Handler;


fn main() {
    // Load token from environment variables or .env file
    let token: String = dotenv::var("DISCORD_TOKEN").expect("token");
    let prefix_aliases: String = dotenv::var("BOT_PREFIXES").expect("prefixes");

    pretty_env_logger::init();

    let mut client = Client::new(
        &token,
        Zero2Handler
    ).expect("Error creating client");

    let owner = match http::get_current_application_info() {
        Ok(info) => info.owner,
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    let mut owner_ids_set = HashSet::new();
    owner_ids_set.insert(owner.id);

    {
        let mut data = client.data.lock();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<MessagePaginator>(HashMap::default());
        data.insert::<BotOwnerContainer>(owner);
    }

    let mut framework = StandardFramework::new()
        .before(|_, msg, _| { let _ = msg.channel_id.broadcast_typing(); true })
        .configure(|c| c
            .prefixes(prefix_aliases.split(','))
            .owners(owner_ids_set))
        ;

    framework = commands::register(framework);

    client.with_framework(framework);

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
