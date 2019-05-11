use serenity::prelude::Context;
use serenity::model::channel::Message;

use crate::core::consts::PREFIX;

mod anilist;


pub fn run_monitors(context: &Context, message: &Message) {
    if !message.author.bot && !message.content_safe(&context.cache).as_str().starts_with(PREFIX.as_str()){
        anilist::anilist_links_monitor(context, message);
        //anilist::rem_monitor(&context, &message);
    }
}
