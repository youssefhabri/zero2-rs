use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
fn next(context: &mut Context, message: &Message) -> CommandResult {
    let _ = message.delete(&context);
    let _ = message.channel_id.send_message(&context.http, |m| {
        m.content(format!("Another satisfied customer! NEXT!"))
    });

    Ok(())
}
