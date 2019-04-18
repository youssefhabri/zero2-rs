use serenity::prelude::Context;
use serenity::model::channel::Message;

use crate::core::consts::PREFIX;

mod anilist;


pub fn run_monitors(ctx: &Context, message: &Message) {
    if !message.author.bot && !message.content_safe().as_str().starts_with(PREFIX.as_str()){
        anilist::anilist_links_monitor(ctx, message);
        //anilist::rem_monitor(&ctx, &message);
    }
}
