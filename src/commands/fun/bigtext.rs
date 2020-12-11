use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
#[aliases("bt")]
async fn bigtext(context: &Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to input a message to convert"));
    }

    let full_message = args.message();
    let bigtext_message = convert_to_bigtext(full_message);

    let _ = message.channel_id.say(&context, bigtext_message).await?;

    Ok(())
}

fn convert_to_bigtext(input: &str) -> String {
    input
        .chars()
        .map(char_to_emoji)
        .collect::<Vec<String>>()
        .join("")
}

fn char_to_emoji(c: char) -> String {
    match c {
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
    }
}
