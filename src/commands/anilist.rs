use serenity::{
    prelude::*,
    client::CACHE,
    framework::StandardFramework,
    framework::standard::{Args, Command, CommandError},
    model::channel::{Message, Reaction},
    model::id::{ChannelId, MessageId},
    builder::{CreateMessage, CreateEmbed}
};

use crate::store::{GuildPaginator, GuildPagination};
use crate::commands::anilist::models::media::Media;

pub mod utils;
pub mod models;
pub mod client;


pub fn register(framework: StandardFramework) -> StandardFramework {
    framework
        .command("anime", |c| c.cmd(AnimeCommand))
}

pub struct AnimeCommand;

impl AnimeCommand {
    fn embed_builder(message: CreateMessage, anime: &Media) -> CreateMessage {
        message.embed(|e| e
            .color(3447003)
            .title(&anime.title.user_preferred)
            .url(&anime.site_url)
            .description(&anime.synopsis())
            .image(&anime.banner_image())
            .thumbnail(&anime.cover_image.large)
            .footer(|f| f
                .text(format!("Status: {} | Powered by AniList", &anime.status())))
            .field("Score", &anime.mean_score(), true)
            .field("Episodes", &anime.episodes(), true)
            .field("Streaming Services", &anime.streaming_services(), true)
            .field("More info", &anime.tracking_sites(), true)
        )
    }
}

impl Command for AnimeCommand {
    fn execute(&self, _context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {
        if args.full().len() <= 0 {
            message.channel_id.say("You need to input an anime title.");
            return Ok(());
        }

        let keyword = args.full().to_owned();

        let results: Vec<Media> = client::search_media(keyword.clone(), "ANIME".to_owned());

        if results.len() > 0 {
            let anime: &Media = results.get(0).unwrap();
            message.channel_id.send_message(
                |m| AnimeCommand::embed_builder(m, anime));
        } else {
            message.channel_id.say(format!("No anime was found for: `{}`", keyword));
        }

        Ok(())
    }
}