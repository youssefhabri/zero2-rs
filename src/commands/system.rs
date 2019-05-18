use std::collections::HashMap;

use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;
use serenity::model::id::MessageId;
use serenity::prelude::*;
use serenity::utils::parse_channel;

use crate::checks::*;
use crate::core::store::{Command, CommandLogger};

#[command("log")]
#[checks(Admin)]
fn log_command(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let mut cmds: HashMap<MessageId, Command> = HashMap::default();

    {
        let data = context.data.read();
        let cmd_logger = data.get::<CommandLogger>().unwrap();
        cmds.clone_from(cmd_logger);
    }

    // TODO only get the last 10-20 commands
    let cmd_log = cmds
        .iter()
        .map(|(_, cmd)| format!("[<@{}>] {}: {}", cmd.user_id, cmd.name, cmd.message))
        .collect::<Vec<String>>()
        .join("\n");

    let _ = message
        .channel_id
        .send_message(&context.http, |m| m.embed(|e| e.description(cmd_log)));

    Ok(())
}

#[command("echo")]
#[owners_only]
fn echo_command(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let _ = message
            .channel_id
            .say(&context.http, "You need to input a channel & a message.");

        return Ok(());
    }

    let segments: Vec<&str> = args.message().split(" ").collect();
    let channel = parse_channel(segments[0]);

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
            message.channel_id.say(&context.http, message_text)?;

            return Ok(());
        }
    } else {
        if segments.len() >= 1 && channel.is_none() {
            // send message to current channel
            message.channel_id.say(&context.http, segments.join(" "))?;
        }
    }

    Ok(())
}

group!({
    name: "System",
    commands: [echo, log]
});
