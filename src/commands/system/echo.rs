use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::{ChannelId, Message};
use serenity::prelude::Context;
use serenity::utils::parse_channel;

use crate::checks::*;

#[command]
#[checks(Admin)]
fn echo(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let _ = message
            .channel_id
            .say(&context.http, "You need to input a channel & a message.");

        return Ok(());
    }

    let segments: Vec<&str> = args.message().split(' ').collect();

    if let Some(channel_id) = parse_channel(segments[0]) {
        if segments.len() == 1 {
            message.channel_id.say(
                &context.http,
                format!("You need to type a message to send to <#{}>", channel_id),
            )?;

            return Ok(());
        }

        if segments.len() > 1 {
            // Send message to channel
            let message_text = segments[1..].join(" ");
            ChannelId(channel_id).say(&context.http, message_text)?;

            return Ok(());
        }
    } else if !segments.is_empty() {
        // send message to current channel
        message.channel_id.say(&context.http, segments.join(" "))?;
    }

    message.delete(context)?;

    Ok(())
}
