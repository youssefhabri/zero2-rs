use reqwest::Client;
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::utils::Colour;
use urbandictionary::ReqwestUrbanDictionaryRequester;

use menu::urban::UrbanDictionaryPagination;

#[command]
async fn urban(context: &Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to input a keyword"));
    }

    let keyword = args.message().to_string();
    let client = Client::new();
    let definitions = client
        .definitions(&keyword)
        .await
        .map_err(|_| CommandError::from("Error requesting Urban Dictionary's definition"))?
        .definitions;

    if definitions.is_empty() {
        let _ = message.channel_id.send_message(&context.http, |f| {
            f.embed(|m| {
                m.color(Colour::GOLD)
                    .title(format!("Could not find \"{}\"", keyword))
                    .description(format!(
                        "Could not find \"{}\" on Urban Dictionary. Are you \
                         sure you wrote it correctly?",
                        keyword
                    ))
            })
        });

        return Ok(());
    }

    UrbanDictionaryPagination::init(&context, &message, definitions).await
}
