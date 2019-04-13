use regex::Regex;
use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::menu::builders;


pub struct ActivityCommand;

impl Command for ActivityCommand {
    fn execute(&self, _context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {

        if args.full().is_empty() {
            let _ = message.channel_id.say("You need to input a activity url or ID.");
            return Ok(());
        }

        let keyword = args.full().to_owned();


        let re = Regex::new(r"\d+/?>?$").unwrap();

        let activity_id = match re.captures(keyword.as_str()) {
            Some(caps) => {
                match caps.get(0) {
                    Some(activity_id) => activity_id.as_str().replace("/", "").replace(">", ""),
                    None => return Ok(())
                }
            },
            None => return Ok(())
        };

        match client::search_activity(activity_id) {
            Some(activity) => {
                let _ = message.channel_id.send_message(
                    |m| m.embed(|_| builders::activity_embed_builder(&activity))
                );
            },
            None => {
                let _ = message.channel_id.say(format!("No user was found for: `{}`", keyword));
            }
        }

        Ok(())
    }
}