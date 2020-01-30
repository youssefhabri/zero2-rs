use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::core::utils::random_num;

#[command]
#[description = "Find out you fortune. It just might be you lucky day ..."]
fn fortune(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    match random_fortune() {
        Some(fortune) => {
            let _ = message.channel_id.send_message(&context.http, |m| {
                m.embed(|e| {
                    e.color(16711769).field(
                        format!("{}'s fortune!", message.author.name),
                        fortune.message,
                        false,
                    )
                })
            });
        }
        None => {
            let _ = message
                .channel_id
                .say(&context.http, "Couldn't find any fortune for you. Sorry!");
        }
    };

    Ok(())
}

#[derive(Clone, Deserialize, Debug)]
pub struct Fortune {
    pub id: String,
    pub message: String,
}

fn request() -> Option<Vec<Fortune>> {
    let client = reqwest::blocking::Client::new();
    let page = random_num(0, 6);
    let mut response = client
        .get(
            format!(
                "http://fortunecookieapi.herokuapp.com/v1/fortunes?limit=&skip=&page={}",
                page
            )
            .as_str(),
        )
        .send()
        .expect("fortune response");

    let fortune_response: Vec<Fortune> = match response.json() {
        Ok(res) => res,
        Err(why) => {
            error!("Err requesting fortunes: {:?}", why);
            return None;
        }
    };

    Some(fortune_response)
}

fn random_fortune() -> Option<Fortune> {
    match request() {
        Some(fortunes) => Some(fortunes[random_num(0, fortunes.len())].clone()),
        None => None,
    }
}
