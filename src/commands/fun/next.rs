use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;

#[command]
async fn next(context: &Context, message: &Message) -> CommandResult {
    let _ = message.delete(&context).await?;
    let _ = message
        .channel_id
        .say(&context, "Another satisfied customer! NEXT!")
        .await?;

    Ok(())
}
