use regex::Regex;
use serenity::model::{channel::Message, guild::Member, id::GuildId};
use serenity::prelude::Context;

use crate::core::consts::PREFIX;

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
        //anilist::rem_monitor(&context, &message);
        message_id::message_id_monitor(context, message);
    }
}

pub fn new_member_monitors(_context: &Context, _guild_id: GuildId, _new_member: &Member) {
    // Greet the user?
}
