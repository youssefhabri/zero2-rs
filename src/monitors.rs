use serenity::model::prelude::{GuildId, Member, Message};
use serenity::prelude::Context;

pub(crate) mod anilist;

pub fn message_monitors(context: &Context, message: &Message) {
    if !message.author.bot {
        anilist::anilist_links_monitor(context, message);
        //anilist::rem_monitor(&context, &message);
    }
}

pub fn new_member_monitors(context: &Context, guild_id: GuildId, new_member: &Member) {
    // Greet the user?
    // Add user to database
}
