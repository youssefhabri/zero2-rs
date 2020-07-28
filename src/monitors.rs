use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::core::consts::PREFIX;

mod anilist;
mod greeting;
mod message_id;

pub use greeting::new_member_monitors;

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
