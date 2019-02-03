//#![feature(inner_deref)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rust_embed;

use std::collections::{HashSet, HashMap};
use std::sync::Arc;
use serenity::{
    client::Client,
    model::{channel::Reaction, gateway::Ready, event::ResumedEvent},
    framework::standard::{StandardFramework, HelpBehaviour, help_commands},
    http,
    prelude::*,
};

mod store;
mod menu;
mod commands;

use crate::store::{
    BotOwnerContainer,
    CommandCounter,
    MessagePaginator,
    ShardManagerContainer,
};


// Event Handler
pub struct Zero2Handler;

impl EventHandler for Zero2Handler {
    fn reaction_add(&self, context: Context, add_reaction: Reaction) {
        menu::handle_reaction(&context, &add_reaction);
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
    let prefix_aliases: String = dotenv::var("BOT_PREFIXES").expect("prefixes");

    // TODO Fix logger on production env
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
    owner_ids_set.insert(owner.id.clone());

    {
        let mut data = client.data.lock();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<MessagePaginator>(HashMap::default());
        data.insert::<BotOwnerContainer>(owner);
    }

    let mut framework = StandardFramework::new()
        .configure(|c| c
            .prefixes(prefix_aliases.split(","))
            .owners(owner_ids_set))
        .command("ping", |c| c.cmd(commands::meta::Ping))
        .command("gif", |c| c.cmd(commands::giphy::GiphyCommand))
        .command("test", |c| c.cmd(commands::meta::Test))
        .customised_help(help_commands::with_embeds, |c| c
            .individual_command_tip("Hello! こんにちは！Hola! Bonjour! 您好!\n\
                If you want more information about a specific command, just pass the command as argument.")
            .command_not_found_text("Could not find: `{}`.")
            .max_levenshtein_distance(3)
            .lacking_permissions(HelpBehaviour::Hide)
            .lacking_role(HelpBehaviour::Nothing)
            .wrong_channel(HelpBehaviour::Strike)
        );

    framework = commands::anilist::register(framework);

    client.with_framework(framework);

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
