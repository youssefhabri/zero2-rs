use serenity::model::prelude::{ChannelId, GuildId, Member};
use serenity::prelude::Context;

use crate::core::store::BotOwnerContainer;

lazy_static! {
    pub static ref GREETINGS: Greetings = Greetings::new();
}

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
            .for_each(|line| greetings.parse(&line.to_string()));

        dbg!(greetings)
    }

    pub fn parse(&mut self, value: &str) {
        let line = value.trim().replace("\\n", "\n");
        let parts: Vec<&str> = line.split("<SP>").collect();

        let weight = parts[0].parse::<u32>().unwrap();
        self.weights.push(weight);

        self.messages.push(parts[1].to_string());
    }

    pub fn random_greeting(&self) -> String {
        crate::core::utils::random_with_weights(&self.messages, &self.weights).unwrap()
    }
}

pub fn new_member_monitors(context: &Context, guild_id: GuildId, new_member: &Member) {
    let guild = match guild_id.to_guild_cached(context) {
        Some(guild) => guild,
        None => {
            error!("Error getting the guild id");
            return;
        }
    };

    let channel_id: ChannelId = match guild.read().system_channel_id {
        Some(channel_id) => channel_id,
        None => {
            error!("Could not find the id of the system channel");
            return;
        }
    };

    let mut greeting = GREETINGS.random_greeting();

    {
        let context_data = context.data.read();
        let owner = context_data.get::<BotOwnerContainer>().unwrap();

        greeting = greeting.replace("{user}", &new_member.to_string());
        greeting = greeting.replace("{owner}", &owner.to_string());
        greeting = greeting.replace("{guild}", &guild.read().name);
    }

    let _ = channel_id.say(context, greeting);
}
