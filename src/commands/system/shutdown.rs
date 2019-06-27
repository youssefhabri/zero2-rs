use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;

use std::process::exit;

#[command]
#[owners_only]
fn shutdown(_: &mut Context, _: &Message, _: Args) -> CommandResult {
    exit(0);
}
