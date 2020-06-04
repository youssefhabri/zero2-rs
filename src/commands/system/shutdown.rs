use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;

use std::process::exit;

#[command]
#[owners_only]
fn shutdown(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let _ = message.channel_id.say(
        context,
        "Shutting down ... This is not enough to silence me. I will come back. I promise ...",
    );

    exit(0);
}
