use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::menu;
use crate::menu::builders;
use crate::models::anilist::character::Character;

#[command]
#[aliases("c")]
#[usage = "<character name>"]
#[description = "Search for a character in AniList"]
fn character(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let _ = message
            .channel_id
            .say(&context.http, "You need to input a character name.");
        return Ok(());
    }

    let keyword = args.message().to_string();

    let results: Vec<Character> = client::search_characters(keyword.clone());

    if !results.is_empty() {
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
            Ok(sending_msg) => menu::new_pagination(
                context,
                sending_msg.id,
                message.author.id,
                builders::pages_builder::<Character>(results, builders::character_embed_builder),
            ),
            Err(why) => error!("Err sending character embed: {:?}", why),
        }
    } else {
        let _ = message.channel_id.say(
            &context.http,
            format!("No user was found for: `{}`", keyword),
        );
    }

    Ok(())
}
