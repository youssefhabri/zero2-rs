use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::{channel::Message, id::UserId, user::User};
use serenity::prelude::*;

use crate::core::utils::random_num;
use regex::Regex;

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
fn nlimage(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
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
        let re = Regex::new(r"^<@!?\d+>$")?;
        match re.captures(params[1].clone().as_str()) {
            Some(caps) => match caps.get(0) {
                Some(user_id) => {
                    let id = user_id
                        .as_str()
                        .replace("<", "")
                        .replace(">", "")
                        .replace("@", "")
                        .replace("!", "")
                        .parse::<u64>()
                        .unwrap();
                    Some(UserId(id).to_user(&context)?)
                }
                None => None,
            },
            None => None,
        }
    } else {
        None
    };

    let selection: String = selection(&context, message, keyword);

    let image: NLImage = query(selection.clone());
    let image_title = selection.replace("_", " ");

    let _ = message.channel_id.send_message(&context.http, |m| {
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
    });

    Ok(())
}

pub fn query(selection: String) -> NLImage {
    let client = reqwest::blocking::Client::new();
    let mut response = client
        .get(format!("https://nekos.life/api/v2/img/{}", selection).as_str())
        .send()
        .expect("Response");

    let result: NLImage = response.json().expect("json");

    result
}

pub fn selection(context: &Context, message: &Message, keyword: String) -> String {
    let nsfw = message.channel(&context.cache).unwrap().is_nsfw();

    if TYPE_LIST.contains(&keyword.as_str()) || (nsfw && NSFW_LIST.contains(&keyword.as_str())) {
        return keyword;
    }

    (TYPE_LIST[random_num(0, TYPE_LIST.len() - 1)]).to_owned()
}
