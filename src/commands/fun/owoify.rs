use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use uwuifier::{round_up16, uwuify_sse};

#[command]
#[usage = "[input text]"]
#[aliases("owo")]
#[description = "OwOfy your text, cause why not."]
async fn owoify(context: &Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to input text to convert."));
    }

    let mut input = args.message().as_bytes().to_owned();
    input.resize(round_up16(input.len()), 0);

    let mut temp_bytes1 = vec![0u8; input.len() * 16];
    let mut temp_bytes2 = vec![0u8; input.len() * 16];

    let output = uwuify_sse(&input, &mut temp_bytes1, &mut temp_bytes2);
    let output = std::str::from_utf8(output)?;

    let _ = message.delete(&context).await;
    let _ = message
        .channel_id
        .send_message(&context, |m| {
            m.content(format!("<@{}> said: {}", message.author.id, output))
        })
        .await;

    Ok(())
}
