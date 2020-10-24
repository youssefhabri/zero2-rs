use serenity::model::prelude::{Message, Reaction};
use serenity::prelude::Context;

mod anilist;
mod discord;

/// Runs message monitors
pub async fn message_monitor(context: &Context, new_message: &Message) {
    anilist::anilist_links_monitor(&context, &new_message).await;

    // Discord message id and url monitor
    discord::id_mention(&context, &new_message).await;
}

/// Runs reaction monitors
pub async fn reaction_add_monitor(context: &Context, reaction: &Reaction) {
    match reaction.user_id {
        Some(user_id) if *user_id.as_u64() == 510136293968183317 => return,
        _ => {}
    }

    menu::handle_reaction(&context, &reaction).await;
}
