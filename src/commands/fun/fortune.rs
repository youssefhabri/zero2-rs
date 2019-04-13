use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::{Args, Command, CommandError};

use crate::utils::random_num;


pub struct FortuneCommand;

impl Command for FortuneCommand {
    fn execute(&self, _: &mut Context, message: &Message, _: Args) -> Result<(), CommandError> {

        match random_fortune() {
            Some(fortune) => {
                let _ = message.channel_id.send_message(|m| m
                    .embed(|e| e
                        .field(
                            format!("{}'s fortune!", message.author.name),
                            fortune.message,
                            false
                        )
                    )
                );
            },
            None => {
                let _ = message.channel_id.say("Couldn't find any fortune for you. Sorry!");
            }
        };

        Ok(())
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Fortune {
    pub id: String,
    pub message: String
}

fn request() -> Option<Vec<Fortune>> {
    let client = reqwest::Client::new();
    let page = random_num(0, 6);
    let mut response = client.get(
        format!("http://fortunecookieapi.herokuapp.com/v1/fortunes?limit=&skip=&page={}", page).as_str()
    ).send().expect("fortune response");

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
        Some(fortunes) => {
            Some(fortunes[random_num(0, fortunes.len())].clone())
        },
        None => None
    }
}