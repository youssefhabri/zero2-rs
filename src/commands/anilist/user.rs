use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::models::anilist::user::User;
use crate::menu;
use crate::menu::builders;


pub struct UserCommand;

impl Command for UserCommand {
    fn execute(&self, context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {

        if args.full().len() <= 0 {
            let _ = message.channel_id.say("You need to input a username.");
            return Ok(());
        }

        let keyword = args.full().to_owned();

        let results: Vec<User> = client::search_user(keyword.clone());

        if results.len() > 0 {
            let user: &User = results.get(0).unwrap();
            let sending = message.channel_id.send_message(
                |m| m.embed(
                    |_| builders::user_embed_builder(user, format!("Page: {}/{} | ", 1, results.len()))
                ).reactions(menu::reactions::default())
            );

            if let Ok(sending_msg) = sending {
                menu::new_pagination(
                    context,
                    sending_msg.id,
                    message.author.id,
                    builders::user_pages_builder(results, builders::user_embed_builder)
                )
            }

        } else {
            let _ = message.channel_id.say(format!("No user was found for: `{}`", keyword));
        }

        Ok(())
    }
}
