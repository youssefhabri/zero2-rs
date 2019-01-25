//#![feature(inner_deref)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rust_embed;

use std::collections::{HashSet, HashMap};
use std::sync::Arc;
use serenity::{
    client::Client,
    model::{channel::Reaction, gateway::Ready, event::ResumedEvent},
    framework::standard::StandardFramework,
    http,
    prelude::*,
};

mod store;
mod commands;

use crate::store::{CommandCounter, ShardManagerContainer, GuildPaginator};


// Event Handler
pub struct Zero2Handler;

impl EventHandler for Zero2Handler {
    fn reaction_add(&self, _ctx: Context, add_reaction: Reaction) {

    }

    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

fn main() {
    // Load token from environment variables or .env file
    let token: String = dotenv::var("DISCORD_TOKEN").expect("token");
    let prefix: String = dotenv::var("BOT_PREFIX").expect("prefix");

    // TODO Fix logger on production env
    pretty_env_logger::init();

    let mut client = Client::new(
        &token,
        Zero2Handler
    ).expect("Error creating client");

    {
        let mut data = client.data.lock();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<GuildPaginator>(HashMap::default());
    }

    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    let mut framework = StandardFramework::new()
        .configure(|c| c
            .prefix(&prefix)
            .owners(owners))
        .command("ping", |c| c.cmd(commands::meta::Ping))
        .command("test", |c| c.cmd(commands::meta::Test));

    framework = commands::anilist::register(framework);

    client.with_framework(framework);

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
