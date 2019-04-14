use serenity::prelude::Context;
use serenity::model::channel::Message;
use regex::Regex;
use crate::models::anilist::media::Media;
use crate::commands::anilist::client;
use crate::menu::builders;
use crate::models::anilist::character::Character;
use crate::models::anilist::user::User;


pub fn rem_monitor(_ctx: &Context, message: &Message) {
    if !message.content_safe().as_str().contains("rem") {
        return
    }

    let _ = message.channel_id.say("Who's rem?");
}

/// AniList Links Monitor
/// 
/// Checks messages for anilist links (containing `https://anilist.co`)
/// and get the data from AniList and embed it in a message.
pub fn anilist_links_monitor(_ctx: &Context, message: &Message) {
    if !message.content_safe().as_str().contains("https://anilist.co/") {
        return
    }

    let full_message = message.content_safe();

    let re = Regex::new(r"https://anilist\.co/(anime|manga|character|activity|user)/([0-9]+)?/?([^/]+)?/?").unwrap();

    for cap in re.captures_iter(full_message.as_str()) {
        match &cap[1] {
            "anime" | "manga" => {
                handle_media(message, &cap[1], &cap[2]);
            },
            "activity" => {
                handle_activity(message, &cap[2]);
            },
            "character" => {
                handle_character(message, &cap[2]);
            },
            "user" => {
                handle_user(message, &cap[3]);
            },
            _ => return
        }
    }
}

/// Handles media embeds for the AniList Links Monitor
fn handle_media(message: &Message, media_type: &str, media_id: &str) {
    let media: Option<Media> = client::search_media_by_id(media_id.into(), media_type.to_uppercase());

    match media {
        Some(media) => {
            let _sending = message.channel_id.send_message(
                |m| m.embed(
                    |_| builders::media_embed_builder(&media, "".into())
                )
            );
        },
        None => return
    }
}

/// Handles activity embeds for the AniList Links Monitor
fn handle_activity(message: &Message, activity_id: &str) {
    match client::search_activity(activity_id.into()) {
        Some(activity) => {
            let _ = message.channel_id.send_message(
                |m| m.embed(|_| builders::activity_embed_builder(&activity))
            );
        },
        None => return
    }
}

/// Handles character embeds for the AniList Links Monitor
fn handle_character(message: &Message, character_id: &str) {
    let character: Option<Character> = client::search_character_by_id(character_id.into());

    match character {
        Some(character) => {
            let _sending = message.channel_id.send_message(
                |m| m.embed(
                    |_| builders::character_embed_builder(&character, "".into())
                )
            );
        },
        None => return
    }
}

/// Handles user embeds for the AniList Links Monitor
fn handle_user(message: &Message, username: &str) {
    let user: Option<User> = client::search_user(username.into());

    match user {
        Some(user) => {
            let _sending = message.channel_id.send_message(
                |m| m.embed(
                    |_| builders::user_embed_builder(&user, "".into())
                )
            );
        },
        None => return
    }
}