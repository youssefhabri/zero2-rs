use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::menu;
use crate::menu::builders;

use crate::models::giphy::*;


pub fn query(query: String) -> GiphyResponse {
    let giphy_key = dotenv::var("GIPHY_API_KEY").expect("giphy_api_token");
    let client = reqwest::Client::new();

    let endpoint = if !query.is_empty() {
        format!("search?q={}", query)
    } else {
        "trending?".to_owned()
    };

    let request = format!("http://api.giphy.com/v1/gifs/{}api_key={}&fmt=json", endpoint, giphy_key);
    let mut res = client.get(request.as_str())
        .send().expect("response");
    let response: GiphyResponse = res.json().expect("json");

    response
}

pub struct GiphyCommand;

impl Command for GiphyCommand {
    fn execute(&self, context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {
        let keyword = args.full().to_owned();
        let results = query(keyword.clone()).data;

        if !results.is_empty() {
            let gif: &Giphy = &results[0];
            let sending = message.channel_id.send_message(
                |m| m.embed(
                    |_| builders::giphy_embed_builder(gif, format!("Page: {}/{} | ", 1, results.len()))
                ).reactions(menu::reactions::default())
            );

            if let Ok(sending_msg) = sending {
                menu::new_pagination(
                    context,
                    sending_msg.id,
                    message.author.id,
                    builders::giphy_pages_builder(results, builders::giphy_embed_builder)
                )
            }

        } else {
            let _ = message.channel_id.say(format!("No gif was found for: `{}`", keyword));
        }

        Ok(())
    }
}