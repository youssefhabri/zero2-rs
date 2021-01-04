use menu::giphy::{GiphyPagination, GiphyResponse};
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[aliases("gif")]
#[usage = "[keyword]"]
async fn giphy(context: &Context, message: &Message, args: Args) -> CommandResult {
    let keyword = args.message().to_string();
    let gifs = query(keyword.clone()).await.data;

    if gifs.is_empty() {
        return Err(CommandError::from(format!(
            "No gif was found for `{}`.",
            keyword
        )));
    }

    GiphyPagination::init(&context, &message, gifs).await
}

pub async fn query(query: String) -> GiphyResponse {
    let giphy_key = kankyo::key("GIPHY_API_KEY").expect("giphy_api_token");
    let client = reqwest::Client::new();

    let endpoint = if !query.is_empty() {
        format!("search?q={}&", query)
    } else {
        "trending?".to_owned()
    };

    let request = format!(
        "http://api.giphy.com/v1/gifs/{}api_key={}&fmt=json",
        endpoint, giphy_key
    );
    let res = client.get(request.as_str()).send().await.expect("response");
    res.json().await.expect("json")
}
