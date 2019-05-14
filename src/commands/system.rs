use crate::core::store::{Command, CommandLogger};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;
use serenity::model::id::MessageId;
use serenity::prelude::*;
use std::collections::HashMap;

use crate::checks::*;

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

group!({
    name: "System",
    commands: [log]
});
