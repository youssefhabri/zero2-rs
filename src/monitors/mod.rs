use serenity::model::prelude::{GuildId, Message, Reaction};
use serenity::prelude::Context;

use crate::utils::{get_global_config_with_default, get_guild_config_with_default};

mod anilist;
mod discord;

async fn can_run_monitor(
    context: &Context,
    guild_id: Option<GuildId>,
    name: &str,
    default: bool,
) -> bool {
    if let Some(guild_id) = guild_id {
        let key = format!("{}_monitor_enable", name);
        let global_config = get_global_config_with_default(&context, key.clone(), default).await;
        let guild_config = get_guild_config_with_default(&context, guild_id, key, default).await;

        return guild_config && global_config;
    }

    default
}

/// Runs message monitors
pub async fn message_monitor(context: &Context, new_message: &Message) {
    if can_run_monitor(&context, new_message.guild_id, "anilist", true).await {
        anilist::anilist_links_monitor(&context, &new_message).await;
    }

    // Discord message id and url monitor
    if can_run_monitor(&context, new_message.guild_id, "discord", true).await {
        discord::id_mention(&context, &new_message).await;
    }
}

/// Runs reaction monitors
pub async fn reaction_add_monitor(context: &Context, reaction: &Reaction) {
    match reaction.user_id {
        Some(user_id) if *user_id.as_u64() == 510136293968183317 => return,
        _ => {}
    }

    menu::handle_reaction(&context, &reaction).await;
}
