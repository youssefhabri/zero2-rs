use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::channel::Message;
use serenity::prelude::*;
use regex::Regex;
use rand::prelude::*;


#[command("nlowo")]
#[aliases("owo")]
#[usage = "[keyword]"]
#[description = "OwOfy you text, cause why not."]
fn nlowo_command(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let input = args.parse::<String>().unwrap_or_else(|_| "".to_string());
    if input.is_empty() {
        let _ = message.channel_id.say(&context.http, "You need to input text to convert.");
        return Ok(());
    }

    let mut rnd = rand::thread_rng();

    let faces = [" owo ", " UwU ", " >w< ", " ^w^ "];
    let mut text = input
        .replace("ove", "uv")
        .replace("n", "ny")
        .replace("N", "NY")
        .to_owned();
    text = Regex::new(r"[rl]").unwrap().replace_all(text.as_str(), "w").into();
    text = Regex::new(r"[RL]").unwrap().replace_all(text.as_str(), "W").into();
    text = Regex::new(r"[!]").unwrap().replace_all(text.as_str(), faces[rnd.gen_range(0, 3)]).into();

    let _ = message.delete(&context);
    let _ = message.channel_id.send_message(
        &context.http,
        |m| m.content(format!("<@{}> said: {}", message.author.id, text))
    );

    Ok(())
}
