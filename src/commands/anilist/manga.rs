use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::models::anilist::media::Media;
use crate::menu;
use crate::menu::builders;


pub struct MangaCommand;

impl Command for MangaCommand {
    fn execute(&self, context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {

        if args.full().len() <= 0 {
            let _ = message.channel_id.say("You need to input a manga title.");
            return Ok(());
        }

        let keyword = args.full().to_owned();

        let results: Vec<Media> = client::search_media(keyword.clone(), "MANGA".to_owned());

        if results.len() > 0 {
            let manga: &Media = results.get(0).unwrap();
            let sending = message.channel_id.send_message(
                |m| m.embed(
                    |_| builders::manga_embed_builder(manga, format!("Page: {}/{} | ", 1, results.len()))
                ).reactions(menu::reactions::default())
            );

            if let Ok(sending_msg) = sending {
                menu::new_pagination(
                    context,
                    sending_msg.id,
                    message.author.id,
                    builders::pages_builder::<Media>(results, builders::manga_embed_builder)
                )
            }

        } else {
            let _ = message.channel_id.say(format!("No manga was found for: `{}`", keyword));
        }

        Ok(())
    }
}