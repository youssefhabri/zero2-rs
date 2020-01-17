use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::menu;
use crate::menu::builders;
use crate::models::anilist::media::{Media, MediaType};

#[command]
#[aliases("a")]
#[usage = "<anime title>"]
#[description = "Search for an anime in AniList"]
pub fn anime(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let _ = message
            .channel_id
            .say(&context.http, "You need to input a anime title.");
        return Ok(());
    }

    let keyword = args.message().to_string();

    let results: Vec<Media> = client::search_media(keyword.clone(), MediaType::Anime);

    if !results.is_empty() {
        let anime: &Media = &results[0];
        let sending = message.channel_id.send_message(&context.http, |m| {
            m.embed(|e| {
                e.clone_from(&builders::media_embed_builder(
                    anime,
                    format!("Page: {}/{} | ", 1, results.len()),
                ));

                e
            })
            .reactions(menu::reactions::default())
        });

        if let Ok(sending_msg) = sending {
            menu::new_pagination(
                context,
                sending_msg.id,
                message.author.id,
                builders::pages_builder::<Media>(results, builders::media_embed_builder),
            )
        }
    } else {
        let _ = message.channel_id.say(
            &context.http,
            format!("No anime was found for: `{}`", keyword),
        );
    }

    Ok(())
}
