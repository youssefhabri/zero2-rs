use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::core::store::PaginationKind;
use crate::menu;
use crate::menu::builders;
use crate::models::anilist::user::User;

#[command]
#[aliases("u")]
#[usage = "<username>"]
#[description = "Search for a user in AniList"]
fn user(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to input a username."));
    }

    let keyword = args.message().to_string();

    let results: Vec<User> = client::search_users(keyword.clone());

    if results.is_empty() {
        return Err(CommandError(format!(
            "No user was found for `{}`.",
            keyword
        )));
    }

    let user: &User = &results[0];
    let sending = message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.clone_from(&builders::user_embed_builder(
                user,
                format!("Page: {}/{} | ", 1, results.len()),
            ));

            e
        })
        .reactions(menu::reactions::default())
    });

    if let Ok(sending_msg) = sending {
        menu::new_pagination_with_handler(
            context,
            sending_msg.id,
            message.author.id,
            PaginationKind::User,
            menu::utils::serialize_entries(results),
            None,
        )
    }

    Ok(())
}
