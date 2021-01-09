use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::{Message, MessageId};
use serenity::prelude::Context;

use crate::core::checks::ADMIN_CHECK;

#[command]
#[checks(Admin)]
async fn cleanup(context: &Context, message: &Message, args: Args) -> CommandResult {
    // TODO handle numbers over 100
    // - add a way to filter users
    let count = args.parse::<u64>().unwrap_or(10);

    let msgs_ids: Vec<MessageId> = message
        .channel_id
        .messages(&context.http, |retriever| {
            retriever.before(message.id).limit(count)
        })
        .await?
        .into_iter()
        .map(|msg| msg.id)
        .collect();

    message
        .channel_id
        .delete_messages(&context, msgs_ids)
        .await?;

    message.delete(&context).await?;

    Ok(())
}
