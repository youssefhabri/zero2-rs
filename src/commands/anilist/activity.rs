use regex::Regex;
use serenity::framework::standard::{
    Args,
    CommandResult,
    macros::command
};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::menu::builders;


#[command("activity")]
#[aliases("act")]
#[usage = "<activity_id|activity_url>"]
#[description = "Embed an activity from AniList"]
fn activity_command(context: &mut Context, message: &Message, args: Args) -> CommandResult {

    if args.is_empty() {
        let _ = message.channel_id.say(&context.http, "You need to input a activity url or ID.");
        return Ok(());
    }

    let keyword = args.message().to_string();

    let re = Regex::new(r"\d+/?>?$").unwrap();

    let activity_id = match re.captures(keyword.as_str()) {
        Some(caps) => {
            match caps.get(0) {
                Some(activity_id) => activity_id
                    .as_str().replace("/", "").replace(">", ""),
                None => return Ok(())
            }
        },
        None => return Ok(())
    };

    match client::search_activity(activity_id) {
        Some(activity) => {
            let _ = message.channel_id.send_message(
                &context.http,
                |m| m.embed(|e| {
                    e.clone_from(&builders::activity_embed_builder(&activity));

                    e
                })
            );
        },
        None => {
            let _ = message.channel_id.say(
                &context.http, format!("No user was found for: `{}`", keyword));
        }
    }

    Ok(())
}
