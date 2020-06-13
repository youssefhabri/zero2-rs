use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use reqwest::blocking::Client as ReqwestClient;

use crate::core::store::PaginationKind;
use crate::menu;
use crate::models::anilist::source::{Response, SourceContainer};

const BASE_URL: &str = "https://trace.moe/api/search";

// TODO add support for attachements

#[command]
#[aliases("sause")]
#[usage = "<image_url>"]
#[description = "Try to find the source of an anime image"]
fn source(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.len() < 1 {
        let _ = message
            .channel_id
            .say(context, "You need to pass an image url.");
        return Ok(());
    }

    let client = ReqwestClient::new();

    let image_url = args.message().trim_matches(|c| c == '<' || c == '>');
    let response = client
        .get(&format!("{}?url={}", BASE_URL, image_url))
        .send()?;

    let mut results: Response = response.json()?;
    results.docs.dedup();

    let containers: Vec<SourceContainer> = results
        .docs
        .iter()
        .map(SourceContainer::from_source)
        .collect();

    if !containers.is_empty() {
        let container: &SourceContainer = &containers[0];
        let sending = message.channel_id.send_message(&context.http, |m| {
            m.embed(|e| {
                e.clone_from(&menu::builders::source_embed_builder(
                    container,
                    format!("Page: {}/{} | ", 1, containers.len()),
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
                PaginationKind::Source,
                menu::utils::serialize_entries(containers),
                None,
            )
        }
    } else {
        let _ = message
            .channel_id
            .say(&context.http, format!("No source was found for the image."));
    }

    Ok(())
}
