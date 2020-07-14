use serenity::model::prelude::{ChannelId, GuildId, Member, Message};
use serenity::prelude::Context;

use crate::core::consts::{GREETINGS, PREFIX};
use crate::core::{store::BotOwnerContainer, utils::random_num};

mod anilist;
mod message_id;

pub fn message_monitors(context: &Context, message: &Message) {
    if !message.author.bot
        && !message
            .content_safe(&context.cache)
            .as_str()
            .starts_with(PREFIX.as_str())
    {
        anilist::anilist_links_monitor(context, message);
        message_id::message_id_monitor(context, message);
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

    let idx = random_num(0, GREETINGS.len() - 1);
    let mut greeting = match GREETINGS.get(idx) {
        Some(greeting) => greeting.to_owned(),
        None => return,
    };

    {
        let context_data = context.data.read();
        let owner = context_data.get::<BotOwnerContainer>().unwrap();

        greeting = greeting.replace("{user}", &new_member.to_string());
        greeting = greeting.replace("{owner}", &owner.to_string());
        greeting = greeting.replace("{guild}", &guild.read().name);
    }

    let _ = channel_id.say(context, greeting);
}
