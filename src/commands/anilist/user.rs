use serenity::framework::standard::{
    Args,
    CommandResult,
    macros::command
};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::models::anilist::user::User;
use crate::menu;
use crate::menu::builders;


#[command("user")]
#[aliases("c")]
#[usage = "<username>"]
#[description = "Search for a user in AniList"]
fn user_command(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let keyword = args.parse::<String>().unwrap_or_else(|_| "".to_string());

    if keyword.is_empty() {
        let _ = message.channel_id.say(&context.http, "You need to input a username.");
        return Ok(());
    }

    let results: Vec<User> = client::search_users(keyword.clone());

    if !results.is_empty() {
        let user: &User = &results[0];
        let sending = message.channel_id.send_message(
            &context.http,
            |m| m.embed(
                |e| {
                    e.clone_from(
                        &builders::user_embed_builder(user, format!("Page: {}/{} | ", 1, results.len()))
                    );

                    e
                }
            ).reactions(menu::reactions::default())
        );

        if let Ok(sending_msg) = sending {
            menu::new_pagination(
                context,
                sending_msg.id,
                message.author.id,
                builders::pages_builder::<User>(results, builders::user_embed_builder)
            )
        }

    } else {
        let _ = message.channel_id.say(&context.http, format!("No user was found for: `{}`", keyword));
    }

    Ok(())
}
