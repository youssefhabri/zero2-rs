use rand::Rng;
use serde::Deserialize;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::{channel::Message, id::UserId, user::User};
use serenity::prelude::Context;
use serenity::utils::parse_username;

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
#[usage = "[keyword:optional] [user:optional]"]
#[description = "Get gifs from nekos.life."]
async fn nlimage(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let params = if !args.is_empty() {
        let mut list: Vec<String> = Vec::new();

        for arg in args.iter::<String>() {
            if let Ok(arg) = arg {
                list.push(arg);
            }
        }

        list
    } else {
        vec![]
    };

    let keyword = if !params.is_empty() {
        params[0].clone()
    } else {
        String::new()
    };
    let user: Option<User> = if params.len() > 1 {
        match parse_username(&params[1]).map(UserId) {
            Some(user_id) => user_id.to_user(&context).await.ok(),
            None => None,
        }
    } else {
        None
    };

    let selection: String = selection(&context, message, keyword).await;

    let image: NLImage = query(selection.clone()).await;
    let image_title = selection.replace("_", " ");

    let _ = message
        .channel_id
        .send_message(&context.http, |m| {
            m.embed(|embed| {
                embed.image(image.url.clone());

                if let Some(user) = user {
                    let _ = message.delete(&context);
                    let _ = message.channel_id.send_message(&context.http, |m| {
                        m.content(format!(
                            "<@{}>: <@{}> sent you a {}",
                            user.id, message.author.id, image_title
                        ))
                    });
                } else {
                    embed.url(image.url).title(image_title);
                }

                embed
            })
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
