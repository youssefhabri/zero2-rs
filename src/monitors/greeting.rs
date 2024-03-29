use once_cell::sync::Lazy;
use serenity::model::prelude::{ChannelId, GuildId, Member};
use serenity::prelude::Context;

use crate::core::consts::OWNER_ID;

pub static GREETINGS: Lazy<Greetings> = Lazy::new(|| Greetings::new());

#[derive(Debug, Default)]
pub struct Greetings {
    messages: Vec<String>,
    weights: Vec<u32>,
}

impl Greetings {
    pub fn new() -> Greetings {
        let mut greetings = Greetings::default();
        include_str!("../../assets/greetings.txt")
            .lines()
            .for_each(|line| greetings.parse(&line));

        greetings
    }

    pub fn parse(&mut self, value: &str) {
        let line = value.trim().replace("\\n", "\n");
        let parts: Vec<&str> = line.split("<SP>").collect();

        let weight = parts[0].parse().unwrap();
        self.weights.push(weight);

        self.messages.push(parts[1].to_string());
    }

    pub fn random_greeting(&self) -> String {
        crate::utils::random_with_weights(&self.messages, &self.weights).unwrap()
    }
}

pub async fn greeting_monitor(context: &Context, guild_id: GuildId, new_member: &Member) {
    let guild = match guild_id.to_guild_cached(context) {
        Some(guild) => guild,
        None => {
            error!("Error getting the guild from the cache");
            return;
        }
    };

    let channel_id: ChannelId = match guild.system_channel_id {
        Some(channel_id) => channel_id,
        None => {
            error!("Could not find the id of the system channel");
            return;
        }
    };

    let owner = format!("<@{}>", OWNER_ID);
    let greeting = GREETINGS
        .random_greeting()
        .replace("{user}", &new_member.to_string())
        .replace("{owner}", &owner)
        .replace("{guild}", &guild.name);

    if let Err(why) = channel_id.say(context, greeting).await {
        error!("Error sending the greeting message: {}", why);
    }
}
