use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::core::store::PaginationKind;
use crate::menu;
use crate::menu::builders;
use crate::models::anilist::staff::Staff;

#[command]
#[aliases("s")]
#[usage = "<staff name>"]
#[description = "Search for a staff in AniList"]
fn staff(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to input a staff name."));
    }

    let keyword = args.message().to_string();

    let results: Vec<Staff> = client::search_staff(keyword.clone());

    if results.is_empty() {
        return Err(CommandError(format!(
            "No staff was found for `{}`.",
            keyword
        )));
    }
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
        menu::new_pagination_with_handler(
            context,
            sending_msg.id,
            message.author.id,
            PaginationKind::Staff,
            menu::utils::serialize_entries(results),
            None,
        )
    }

    Ok(())
}
