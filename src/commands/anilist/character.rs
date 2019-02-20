use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::models::anilist::character::Character;
use crate::menu;
use crate::menu::builders;


pub struct CharacterCommand;

impl Command for CharacterCommand {
    fn execute(&self, context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {

        if args.full().len() <= 0 {
            let _ = message.channel_id.say("You need to input a character name.");
            return Ok(());
        }

        let keyword = args.full().to_owned();

        let results: Vec<Character> = client::search_characters(keyword.clone());

        if results.len() > 0 {
            let character: &Character = results.get(0).unwrap();
            let sending = message.channel_id.send_message(
                |m| m.embed(
                    |_| builders::character_embed_builder(character, format!("Page: {}/{} | ", 1, results.len()))
                ).reactions(menu::reactions::default())
            );

            match sending {
                Ok(sending_msg) => menu::new_pagination(
                    context,
                    sending_msg.id,
                    message.author.id,
                    builders::character_pages_builder(results, builders::character_embed_builder)
                ),
                Err(why) => error!("Err sending character embed: {:?}", why)
            }

        } else {
            let _ = message.channel_id.say(format!("No user was found for: `{}`", keyword));
        }

        Ok(())
    }
}