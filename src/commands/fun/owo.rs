use rand::prelude::*;
use regex::Regex;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[usage = "[keyword]"]
#[description = "OwOfy you text, cause why not."]
fn owo(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let _ = message
            .channel_id
            .say(&context.http, "You need to input text to convert.");
        return Ok(());
    }

    let input = args.message().to_string();

    let mut rnd = rand::thread_rng();

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
        .replace_all(text.as_str(), faces[rnd.gen_range(0, 3)])
        .into();

    let _ = message.delete(&context);
    let _ = message.channel_id.send_message(&context.http, |m| {
        m.content(format!("<@{}> said: {}", message.author.id, text))
    });

    Ok(())
}
