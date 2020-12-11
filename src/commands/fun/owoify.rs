use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::utils::random_number;

#[command]
#[usage = "[input text]"]
#[aliases("owo")]
#[description = "OwOfy your text, cause why not."]
async fn owoify(context: &Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to input text to convert."));
    }

    let faces = [" OwO ", " UwU ", " >w< ", " ^w^ "];

    let input = args.message().to_string();
    let output = input
        .replace("ove", "uv")
        .replace("n", "ny")
        .replace("N", "NY")
        .replace(&['r', 'l'][..], "w")
        .replace(&['R', 'L'][..], "W")
        .replace('!', faces[random_number(0, 3)]);

    let _ = message.delete(&context).await;
    let _ = message
        .channel_id
        .send_message(&context, |m| {
            m.content(format!("<@{}> said: {}", message.author.id, output))
        })
        .await;

    Ok(())
}
