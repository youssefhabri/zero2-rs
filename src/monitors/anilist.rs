use regex::Regex;
use serenity::model::channel::Message;
use serenity::prelude::Context;

use crate::commands::anilist::client;
use crate::match_send;
use crate::menu::builders;
use crate::models::anilist::{
    character::Character, media::Media, staff::Staff, studio::Studio, user::User,
};

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"https://anilist\.co/(anime|manga|character|activity|user|studio|staff)/([0-9]+)?/?([^/]+)?/?",
    )
    .unwrap();
}

pub fn _rem_monitor(context: &Context, message: &Message) {
    if !message
        .content_safe(&context.cache)
        .as_str()
        .contains("rem")
    {
        return;
    }

    let _ = message.channel_id.say(&context.http, "Who's rem?");
}

fn should_embed(message: &str) -> bool {
    message.contains("https://anilist.co/")
        && (!message.contains("noembed") || !message.contains("-ne"))
}

/// AniList Links Monitor
///
/// Checks messages for anilist links (containing `https://anilist.co`)
/// and get the data from AniList and embed it in a message.
pub fn anilist_links_monitor(context: &Context, message: &Message) {
    let full_message = message.content_safe(&context.cache);

    if !should_embed(&full_message.as_str()) {
        return;
    }

    let matches: Vec<_> = RE.captures_iter(full_message.as_str()).collect();

    if matches.len() != 1 {
        return;
    }

    let cap = &matches[0];

    match &cap[1] {
        "anime" | "manga" => {
            handle_media(context, message, &cap[1], &cap[2]);
        }
        "activity" => {
            handle_activity(context, message, &cap[2]);
        }
        "character" => {
            handle_character(context, message, &cap[2]);
        }
        "user" => {
            handle_user(context, message, &cap[3]);
        }
        "studio" => {
            handle_studio(context, message, &cap[2]);
        }
        "staff" => {
            handle_staff(context, message, &cap[2]);
        }
        _ => {}
    }
}

/// Handles media embeds for the AniList Links Monitor
fn handle_media(context: &Context, message: &Message, media_type: &str, media_id: &str) {
    let media: Option<Media> =
        client::search_media_by_id(media_id.to_string(), media_type.to_uppercase());

    match_send!(context, message, media, builders::media_embed_builder);
}

/// Handles activity embeds for the AniList Links Monitor
fn handle_activity(context: &Context, message: &Message, activity_id: &str) {
    let activity = client::search_activity(activity_id.into());

    match_send!(context, message, activity, builders::activity_embed_builder);
}

/// Handles character embeds for the AniList Links Monitor
fn handle_character(context: &Context, message: &Message, character_id: &str) {
    let character: Option<Character> = client::search_character_by_id(character_id.into());

    match_send!(
        context,
        message,
        character,
        builders::character_embed_builder
    );
}

/// Handles user embeds for the AniList Links Monitor
fn handle_user(context: &Context, message: &Message, username: &str) {
    let user: Option<User> = client::search_user(username.into());

    match_send!(context, message, user, builders::user_embed_builder);
}

/// Handles studio embeds for the AniList Links Monitor
fn handle_studio(context: &Context, message: &Message, studio_id: &str) {
    let studio: Option<Studio> = client::search_studio(studio_id.into());

    match_send!(context, message, studio, builders::studio_embed_builder);
}

/// Handles staff embeds for the AniList Links Monitor
fn handle_staff(context: &Context, message: &Message, staff_id: &str) {
    let staff: Option<Staff> = client::search_staff_by_id(staff_id.into());

    match_send!(context, message, staff, builders::staff_embed_builder);
}

#[macro_export]
macro_rules! match_send {
    ($context:expr, $message:expr, $data:expr, $embed_builder:expr) => {
        if let Some(data) = $data {
            let _sending = ($message).channel_id.send_message(&($context).http, |m| {
                m.embed(|embed| {
                    embed.clone_from(&($embed_builder(&data, "".into())));

                    embed
                })
            });
        }
    };
}
