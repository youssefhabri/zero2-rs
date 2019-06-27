use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::core::{consts::COOKIES, utils::random_num};

#[command]
fn cookie(context: &mut Context, message: &Message, _args: Args) -> CommandResult {
    if !&COOKIES.is_empty() {
        message.channel_id.send_message(&context.http, |m| {
            m.embed(|e| {
                e.color(16711769).field(
                    format!("{}'s fortune cookie!", message.author.name),
                    &COOKIES[random_num(0, COOKIES.len() - 1)],
                    false,
                )
            })
        })?;

        return Ok(());
    }

    let _ = message.channel_id.say(
        &context.http,
        "Couldn't find any fortune cookies for you. Sorry!",
    );

    Ok(())
}
