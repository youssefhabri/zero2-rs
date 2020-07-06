use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::{ChannelId, Message};
use serenity::prelude::Context;
use serenity::utils::parse_channel;

#[command]
#[owners_only]
fn echo(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from(
            "You need to input a channel & a message.",
        ));
    }

    let segments: Vec<&str> = args.message().split(' ').collect();

    match parse_channel(segments[0]) {
        Some(channel_id) if segments.len() == 1 => {
            return Err(CommandError(format!(
                "You need to type a message to send to <#{}>",
                channel_id
            )))
        }
        Some(channel_id) if segments.len() > 1 => {
            // Send message to channel
            let message_text = segments[1..].join(" ");
            ChannelId(channel_id).say(&context.http, message_text)?;
        }
        None if !segments.is_empty() => {
            // send message to current channel
            message.channel_id.say(&context.http, segments.join(" "))?;
        }
        _ => {}
    };

    message.delete(context)?;

    Ok(())
}
