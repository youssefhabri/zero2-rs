use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::{ChannelId, Message};
use serenity::prelude::Context;
use serenity::utils::parse_channel;

use crate::checks::*;

#[command]
#[checks(Admin)]
fn embed(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let full_message: String = args.message().to_string();

    let segments: Vec<&str> = full_message.split(" | ").collect();

    if segments.len() < 3 {
        let _ = message
            .channel_id
            .say(&context.http, "This commands need 3 arguments");
        return Ok(());
    }

    let channel = match parse_channel(segments[0]) {
        Some(cid) => ChannelId(cid),
        None => {
            let _ = message.channel_id.say(
                &context.http,
                format!("{} is not a valid channel.", segments[0]),
            );

            return Ok(());
        }
    };

    let _ = channel.send_message(&context.http, |m| {
        m.embed(|e| e.title(segments[1]).description(segments[2..].join(" ")))
    });

    Ok(())
}
