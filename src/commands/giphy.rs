use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::menu;
use crate::menu::builders;

use crate::models::giphy::*;

pub fn query(query: String) -> GiphyResponse {
    let giphy_key = dotenv::var("GIPHY_API_KEY").expect("giphy_api_token");
    let client = reqwest::blocking::Client::new();

    let endpoint = if !query.is_empty() {
        format!("search?q={}&", query)
    } else {
        "trending?".to_owned()
    };

    let request = format!(
        "http://api.giphy.com/v1/gifs/{}api_key={}&fmt=json",
        endpoint, giphy_key
    );
    let mut res = client.get(request.as_str()).send().expect("response");
    let response: GiphyResponse = res.json().expect("json");

    response
}

#[command]
#[aliases("gif")]
#[usage = "[keyword]"]
fn giphy(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let keyword = args.message().to_string();
    let results = query(keyword.clone()).data;

    if !results.is_empty() {
        let gif: &Giphy = &results[0];
        let sending = message.channel_id.send_message(&context.http, |m| {
            m.embed(|e| {
                e.clone_from(&builders::giphy_embed_builder(
                    gif,
                    format!("Page: {}/{} | ", 1, results.len()),
                ));

                e
            })
            .reactions(menu::reactions::default())
        });

        if let Ok(sending_msg) = sending {
            menu::new_pagination(
                context,
                sending_msg.id,
                message.author.id,
                builders::giphy_pages_builder(results, builders::giphy_embed_builder),
            )
        }
    } else {
        let _ = message.channel_id.say(
            &context.http,
            format!("No gif was found for: `{}`", keyword),
        );
    }

    Ok(())
}
