use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[aliases("bt")]
fn bigtext(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        let _ = message
            .channel_id
            .say(&context.http, "You need to input a message to convert");
        return Ok(());
    }

    let full_message: String = args.message().to_string();
    let bigtext_message: String = text_to_bigtext(full_message);

    let _ = message.channel_id.say(&context.http, bigtext_message);

    Ok(())
}

fn text_to_bigtext(text: String) -> String {
    text.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' => format!(":regional_indicator_{}:", c.to_lowercase()),
            '0'..='9' => match c {
                '0' => ":zero:",
                '1' => ":one:",
                '2' => ":two:",
                '3' => ":three:",
                '4' => ":four:",
                '5' => ":five:",
                '6' => ":six:",
                '7' => ":seven:",
                '8' => ":eight:",
                '9' => ":nine:",
                _ => "",
            }
            .to_string(),
            ' ' => "  ".to_string(),
            '!' => ":exclamation:".to_string(),
            '?' => ":question:".to_string(),
            '+' => ":heavy_plus_sign:".to_string(),
            '-' => ":heavy_minus_sign:".to_string(),
            '.' => ":small_blue_diamond:".to_string(),
            _ => c.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}
