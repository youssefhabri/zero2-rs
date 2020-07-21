use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use reqwest::blocking::Client as ReqwestClient;

use crate::core::store::PaginationKind;
use crate::menu;
use crate::models::anilist::source::{Body, Response, SourceContainer};

const BASE_URL: &str = "https://trace.moe/api/search";

// TODO source for last image in chat
// TODO source from a message id
// TODO I need to add a way to check the ratelimit

#[command]
#[aliases("sause")]
#[usage = "<image_url>"]
#[description = "Try to find the source of an anime image"]
fn source(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let client = ReqwestClient::new();

    let response = if args.is_empty() {
        if message.attachments.is_empty() || message.attachments[0].dimensions().is_none() {
            return Err(CommandError::from(
                "You need to pass an image url or upload an image.",
            ));
        }

        match message.attachments[0].download() {
            Ok(image_data) => {
                let body = Body {
                    image: format!("data:image/jpeg;base64,{}", base64::encode(&image_data)),
                };

                client.post(BASE_URL).json(&body).send()?
            }
            Err(_why) => {
                return Err(CommandError::from("Error while processing your request."));
            }
        }
    } else {
        let image_url = args.message().trim_matches(|c| c == '<' || c == '>');
        client
            .get(&format!("{}?url={}", BASE_URL, image_url))
            .send()?
    };

    let mut results: Response = response.json()?;
    results.docs.dedup();

    let containers: Vec<SourceContainer> = results
        .docs
        .iter()
        .filter(|source| source.similarity >= 0.87)
        .map(SourceContainer::from_source)
        .collect();

    if containers.is_empty() {
        return Err(CommandError::from("No source was found for the image."));
    }
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

    Ok(())
}
