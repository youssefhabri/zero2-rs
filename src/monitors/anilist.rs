use serenity::prelude::Context;
use serenity::model::channel::Message;
use regex::Regex;
use crate::models::anilist::media::Media;
use crate::commands::anilist::client;
use crate::menu::builders;
use crate::models::anilist::character::Character;
use crate::models::anilist::user::User;


pub fn anilist_monitor(_ctx: Context, message: Message) {
    if !message.content_safe().as_str().contains("https://anilist.co/") {
        return
    }

    let full_message = message.content_safe();

    let re = Regex::new(r"https://anilist\.co/(anime|manga|character|activity|user)/([0-9]+)?/?([^/]+)?/?").unwrap();

    let (
        _group0, // URI
        group1,  // TYPE
        group2,  // ID
        group3   // TITLE | USERNAME
    ) = match re.captures(full_message.as_str()) {
        Some(caps) => {
            let group0 = match caps.get(0) {
                Some(group0) => Some(group0.as_str()),
                None => None
            };

            let group1 = match caps.get(1) {
                Some(group1) => Some(group1.as_str()),
                None => None
            };

            let group2 = match caps.get(2) {
                Some(group2) => Some(group2.as_str()),
                None => None
            };

            let group3 = match caps.get(3) {
                Some(group3) => Some(group3.as_str()),
                None => None
            };

            (group0, group1, group2, group3)
        },
        None => {
            debug!("Pattern matching failed!");
            return
        }
    };

    match group1.unwrap() {
        "anime" | "manga" => {
            handle_media(message, group1.unwrap(), group2.unwrap());
        },
        "activity" => {
            handle_activity(message, group2.unwrap());
        },
        "character" => {
            handle_character(message, group2.unwrap());
        },
        "user" => {
            handle_user(message, group3.unwrap());
        },
        _ => return
    }
}

fn handle_media(message: Message, media_type: &str, media_id: &str) {
    let media: Option<Media> = client::search_media_by_id(media_id.into(), media_type.to_uppercase());

    match media {
        Some(media) => {
            let _sending = message.channel_id.send_message(
                |m| m.embed(
                    |_| builders::anime_embed_builder(&media, "".into())
                )
            );
        },
        None => return
    }
}

fn handle_activity(message: Message, activity_id: &str) {
    match client::search_activity(activity_id.into()) {
        Some(activity) => {
            let _ = message.channel_id.send_message(
                |m| m.embed(|_| builders::activity_embed_builder(&activity))
            );
        },
        None => return
    }
}

fn handle_character(message: Message, character_id: &str) {
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

fn handle_user(message: Message, username: &str) {
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