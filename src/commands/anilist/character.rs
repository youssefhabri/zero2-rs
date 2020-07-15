use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::core::store::PaginationKind;
use crate::menu;
use crate::menu::builders;
use crate::models::anilist::character::Character;

#[command]
#[aliases("c")]
#[usage = "<character name>"]
#[description = "Search for a character in AniList"]
fn character(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to input a character name."));
    }

    let keyword = args.message().to_string();

    let results: Vec<Character> = client::search_characters(keyword.clone());

    if results.is_empty() {
        return Err(CommandError(format!(
            "No character was found for `{}`.",
            keyword
        )));
    }

    let character: &Character = &results[0];
    let sending = message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.clone_from(&builders::character_embed_builder(
                character,
                format!("Page: {}/{} | ", 1, results.len()),
            ));

            e
        })
        .reactions(menu::reactions::default())
    });

    match sending {
        Ok(sending_msg) => menu::new_pagination_with_handler(
            context,
            sending_msg.id,
            message.author.id,
            PaginationKind::Character,
            menu::utils::serialize_entries(results),
            None,
        ),
        Err(why) => error!("Err sending character embed: {:?}", why),
    }

    Ok(())
}
