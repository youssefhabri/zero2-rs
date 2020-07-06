use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::core::{consts::COOKIES, utils::random_num};

#[command]
fn cookie(context: &mut Context, message: &Message, _args: Args) -> CommandResult {
    if &COOKIES.is_empty() {
        return Err(CommandError::from(
            "Couldn't find any fortune cookies for you. Sorry!",
        ));
    }
    message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.color(16711769)
                .title(format!("{}'s fortune cookie!", message.author.name))
                .description(&COOKIES[random_num(0, COOKIES.len() - 1)].trim())
        })
    })?;

    Ok(())
}
