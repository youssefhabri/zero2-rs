use serenity::prelude::Context;
use serenity::model::channel::Message;

mod anilist;

fn starts_with_prefix(message: &Message) -> bool {
    let prefixes = dotenv::var("BOT_PREFIXES").expect("prefixes");
    for prefix in prefixes.split(",") {
        if message.content_safe().as_str().starts_with(prefix) {
            return true;
        }
    }

    false
}

pub fn run_monitors(ctx: &Context, message: &Message) {
    if !message.author.bot && !starts_with_prefix(message) {
        anilist::anilist_links_monitor(ctx, message);
        //anilist::rem_monitor(&ctx, &message);
    }
}
