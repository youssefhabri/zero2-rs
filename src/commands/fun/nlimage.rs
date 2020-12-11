use rand::Rng;
use serde::Deserialize;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;

const TYPE_LIST: [&str; 17] = [
    "tickle",
    "slap",
    "poke",
    "pat",
    "neko",
    "meow",
    "lizard",
    "kiss",
    "hug",
    "kemonomimi",
    "feed",
    "cuddle",
    "holo",
    "smug",
    "baka",
    "woof",
    "fox_girl",
];

const NSFW_LIST: [&str; 1] = ["ngif"];

#[derive(Deserialize, Debug)]
pub struct NLImage {
    url: String,
}

#[command]
#[aliases("nl", "nlimg")]
#[usage = "[keyword:optional]"]
#[description = "Get gifs from nekos.life."]
async fn nlimage(context: &Context, message: &Message, args: Args) -> CommandResult {
    let keyword = args.message().to_string();
    let selection: String = selection(&context, message, keyword).await;

    let image: NLImage = query(selection.clone()).await;
    let image_title = selection.replace("_", " ");

    let _ = message
        .channel_id
        .send_message(&context.http, |m| {
            m.embed(|embed| embed.image(&image.url).url(image.url).title(image_title))
        })
        .await?;

    Ok(())
}

pub async fn query(selection: String) -> NLImage {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://nekos.life/api/v2/img/{}", selection).as_str())
        .send()
        .await
        .expect("Response");

    response.json().await.expect("json")
}

pub async fn selection(context: &Context, message: &Message, keyword: String) -> String {
    let nsfw = message
        .channel(&context.cache)
        .await
        .map(|ch| ch.is_nsfw())
        .unwrap_or(false);

    if TYPE_LIST.contains(&keyword.as_str()) || (nsfw && NSFW_LIST.contains(&keyword.as_str())) {
        return keyword;
    }

    let random_num: usize = rand::thread_rng().gen_range(0, TYPE_LIST.len() - 1);
    TYPE_LIST[random_num].to_owned()
}
