use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::menu;
use crate::menu::builders;
use crate::models::anilist::staff::Staff;

#[command]
#[aliases("s")]
#[usage = "<staff name>"]
#[description = "Search for a staff in AniList"]
fn staff(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let _ = message
            .channel_id
            .say(&context.http, "You need to input a staff name.");
        return Ok(());
    }

    let keyword = args.message().to_string();

    let results: Vec<Staff> = client::search_staff(keyword.clone());

    if !results.is_empty() {
        let staff: &Staff = &results[0];
        let sending = message.channel_id.send_message(&context.http, |m| {
            m.embed(|e| {
                e.clone_from(&builders::staff_embed_builder(
                    staff,
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
                builders::pages_builder::<Staff>(results, builders::staff_embed_builder),
            )
        }
    } else {
        let _ = message.channel_id.say(
            &context.http,
            format!("No staff was found for: `{}`", keyword),
        );
    }

    Ok(())
}
