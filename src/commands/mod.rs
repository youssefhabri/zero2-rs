use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub mod anilist;

#[command]
async fn test(_context: &Context, _message: &Message, _args: Args) -> CommandResult {
    Ok(())
}

#[group]
#[commands(test)]
struct Root;
