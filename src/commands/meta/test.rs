use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::channel::Message;
use serenity::prelude::*;


#[command("test")]
fn test_command(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let _ = message.channel_id.say(&context.http, "!test");

    Ok(())
}
