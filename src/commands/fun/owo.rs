use regex::Regex;
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::core::utils::random_num;

#[command]
#[usage = "[keyword]"]
#[description = "OwOfy you text, cause why not."]
fn owo(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to input text to convert."));
    }

    let input = args.message().to_string();

    let faces = [" owo ", " UwU ", " >w< ", " ^w^ "];
    let mut text = input
        .replace("ove", "uv")
        .replace("n", "ny")
        .replace("N", "NY");
    text = Regex::new(r"[rl]")
        .unwrap()
        .replace_all(text.as_str(), "w")
        .into();
    text = Regex::new(r"[RL]")
        .unwrap()
        .replace_all(text.as_str(), "W")
        .into();
    text = Regex::new(r"[!]")
        .unwrap()
        .replace_all(text.as_str(), faces[random_num(0, 3)])
        .into();

    let _ = message.delete(&context);
    let _ = message.channel_id.send_message(&context.http, |m| {
        m.content(format!("<@{}> said: {}", message.author.id, text))
    });

    Ok(())
}
