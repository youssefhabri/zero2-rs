use serenity::model::{channel::Message, guild::Member, id::GuildId};
use serenity::prelude::Context;

use crate::core::consts::PREFIX;

mod anilist;

pub fn message_monitors(context: &Context, message: &Message) {
    if !message.author.bot
        && !message
            .content_safe(&context.cache)
            .as_str()
            .starts_with(PREFIX.as_str())
    {
        anilist::anilist_links_monitor(context, message);
        //anilist::rem_monitor(&context, &message);
    }
}

pub fn new_member_monitors(context: &Context, guild_id: GuildId, new_member: &Member) {
    // Greet the user?
}
