use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::{Message, MessageId};
use serenity::prelude::Context;

use crate::checks::*;

#[command]
#[checks(Admin)]
fn cleanup(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    // TODO handle numbers over 100
    //  as discord limit the number to 100
    //  we need to iterate multiple time to handle that
    let msgs_count: u64 = if !args.is_empty() { args.parse()? } else { 10 };

    let msgs_ids: Vec<MessageId> = message
        .channel_id
        .messages(&context.http, |retriever| {
            retriever.before(message.id).limit(msgs_count)
        })?
        .into_iter()
        .map(|msg| msg.id)
        .collect();

    message
        .channel_id
        .delete_messages(&context.http, msgs_ids)?;

    message.delete(&context)?;

    Ok(())
}
