use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use uwuifier::uwuify_str_sse;

#[command]
#[usage = "[input text]"]
#[aliases("owo")]
#[description = "OwOfy your text, cause why not."]
async fn owoify(context: &Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to input text to convert."));
    }

    let input = args.message();
    let output = uwuify_str_sse(input);

    let _ = message.delete(&context).await;
    let _ = message
        .channel_id
        .send_message(&context, |m| {
            m.content(format!("<@{}> said: {}", message.author.id, output))
        })
        .await;

    Ok(())
}
