use serenity::{
    prelude::*,
    framework::standard::{Args, Command, CommandError},
    model::channel::Message,
};
use crate::commands::anilist::models::media::Media;

pub mod models;
pub mod client;


pub struct AnimeCommand;

impl Command for AnimeCommand {
    fn execute(&self, _context: &mut Context, message: &Message, _args: Args) -> Result<(), CommandError> {
        let keyword = "clannad";

        let results: Vec<Media> = client::search_media(keyword.to_owned(), "ANIME".to_owned());

        if results.len() > 0 {
            let anime: &Media = results.get(0).unwrap();
            message.channel_id.say(&anime.title.user_preferred);
        } else {
            message.channel_id.say(format!("No anime was found for: `{}`", keyword));
        }


        Ok(())
    }
}