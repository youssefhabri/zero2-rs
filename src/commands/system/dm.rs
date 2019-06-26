use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
#[owners_only]
fn dm(context: &mut Context, message: &Message, _args: Args) -> CommandResult {
    message
        .author
        .dm(&context, |m| m.content("Hello, there!"))?;

    Ok(())
}
