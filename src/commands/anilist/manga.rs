use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::core::store::PaginationKind;
use crate::menu;
use crate::menu::builders;
use crate::models::anilist::media::{Media, MediaType};

#[command]
#[aliases("m")]
#[usage = "<manga title>"]
#[description = "Search for a manga in AniList"]
fn manga(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to input a manga title."));
    }

    let keyword = args.message().to_string();

    let results: Vec<Media> = client::search_media(keyword.clone(), MediaType::Manga);

    if results.is_empty() {
        return Err(CommandError(format!(
            "No manga was found for `{}`.",
            keyword
        )));
    }

    let manga: &Media = &results[0];
    let sending = message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.clone_from(&builders::media_embed_builder(
                manga,
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
            PaginationKind::Media,
            menu::utils::serialize_entries(results),
            None,
        )
    }

    Ok(())
}
