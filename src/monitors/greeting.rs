use serenity::model::prelude::{ChannelId, GuildId, Member};
use serenity::prelude::Context;

use crate::core::consts::GREETINGS;
use crate::core::utils::random_num;

pub fn new_member_monitors(context: &Context, guild_id: GuildId, new_member: &Member) {
    let idx = random_num(0, GREETINGS.len() - 1);

    let mut greeting = match GREETINGS.get(idx) {
        Some(greeting) => *greeting,
        None => return,
    };

    greeting = greeting.replace("{user}", &new_member.to_string());
    greeting = greeting.replace("{guild}", &guild_id.to_string());

    let channel_id: ChannelId = match guild_id.to_guild_cached(context) {
        Some(guild) => match guild.read().system_channel_id {
            Some(channel_id) => channel_id,
            None => return,
        },
        None => return,
    };

    let _ = channel_id.say(context, greeting);
}
