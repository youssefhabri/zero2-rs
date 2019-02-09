use serenity::{
    prelude::*,
    framework::StandardFramework,
    framework::standard::{Args, Command, CommandError},
    model::channel::Message,
};

use crate::commands::anilist::models::{
    character::Character,
    media::Media,
    user::User
};
use crate::menu::builders;

// Import menu functionality
use crate::menu;
use regex::Regex;

pub mod utils;
pub mod models;
pub mod client;


pub fn register(framework: StandardFramework) -> StandardFramework {
    framework.group("AniList", |cg| cg
        .command("anime", |c| c
            .cmd(AnimeCommand)
            .batch_known_as(vec!["a"])
            .usage("<anime>")
            .desc("Search for an anime in AniList")
        )
        .command("manga", |c| c
            .cmd(MangaCommand)
            .batch_known_as(vec!["m"])
            .usage("<manga>")
            .desc("Search for a manga in AniList")
        )
        .command("user", |c| c
            .cmd(UserCommand)
            .batch_known_as(vec!["u"])
            .usage("<user>")
            .desc("Search for a user in AniList")
        )
        .command("character", |c| c
            .cmd(CharacterCommand)
            .batch_known_as(vec!["c"])
            .usage("<character>")
            .desc("Search for a character in AniList")
        )
        .command("activity", |c| c
            .cmd(ActivityCommand)
            .batch_known_as(vec!["act"])
            .usage("<activity_id|activity_url>")
            .desc("Embed an activity from AniList")
        )
    )
}

pub struct AnimeCommand;

impl Command for AnimeCommand {
    fn execute(&self, context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {

        if args.full().len() <= 0 {
            let _ = message.channel_id.say("You need to input a anime title.");
            return Ok(());
        }

        let keyword = args.full().to_owned();

        let results: Vec<Media> = client::search_media(keyword.clone(), "ANIME".to_owned());

        if results.len() > 0 {
            let anime: &Media = results.get(0).unwrap();
            let sending = message.channel_id.send_message(
                |m| m.embed(
                    |_| builders::anime_embed_builder(anime, format!("Page: {}/{} | ", 1, results.len()))
                ).reactions(menu::REACTIONS.to_vec())
            );

            if let Ok(sending_msg) = sending {
                menu::new_pagination(
                    context,
                    sending_msg.id,
                    message.author.id,
                    builders::media_pages_builder(results, builders::anime_embed_builder)
                )
            }

        } else {
            let _ = message.channel_id.say(format!("No anime was found for: `{}`", keyword));
        }

        Ok(())
    }
}

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
                ).reactions(menu::REACTIONS.to_vec())
            );

            if let Ok(sending_msg) = sending {
                menu::new_pagination(
                    context,
                    sending_msg.id,
                    message.author.id,
                    builders::media_pages_builder(results, builders::manga_embed_builder)
                )
            }

        } else {
            let _ = message.channel_id.say(format!("No manga was found for: `{}`", keyword));
        }

        Ok(())
    }
}

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
                ).reactions(menu::REACTIONS.to_vec())
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

pub struct CharacterCommand;

impl Command for CharacterCommand {
    fn execute(&self, context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {

        if args.full().len() <= 0 {
            let _ = message.channel_id.say("You need to input a character name.");
            return Ok(());
        }

        let keyword = args.full().to_owned();

        let results: Vec<Character> = client::search_character(keyword.clone());

        if results.len() > 0 {
            let character: &Character = results.get(0).unwrap();
            let sending = message.channel_id.send_message(
                |m| m.embed(
                    |_| builders::character_embed_builder(character, format!("Page: {}/{} | ", 1, results.len()))
                ).reactions(menu::REACTIONS.to_vec())
            );

            if let Ok(sending_msg) = sending {
                menu::new_pagination(
                    context,
                    sending_msg.id,
                    message.author.id,
                    builders::character_pages_builder(results, builders::character_embed_builder)
                )
            }

        } else {
            let _ = message.channel_id.say(format!("No user was found for: `{}`", keyword));
        }

        Ok(())
    }
}

pub struct ActivityCommand;

impl Command for ActivityCommand {
    fn execute(&self, _context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {

        if args.full().len() <= 0 {
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
