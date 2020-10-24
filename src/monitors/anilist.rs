use regex::Regex;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use menu::anilist::AniListPagination;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"https://anilist\.co/(anime|manga|character|activity|user|studio|staff)/([0-9]+)?/?([^/]+)?/?",
    )
    .unwrap();
}

macro_rules! ok_or_return {
    ($e:expr) => {
        match $e {
            Ok(value) => value,
            Err(_) => return,
        };
    };
}

fn should_embed(message: &str) -> bool {
    message.contains("https://anilist.co/")
        && (!message.contains("noembed") || !message.contains("-ne"))
}

/// AniList Links Monitor
///
/// Checks messages for anilist links (containing `https://anilist.co`)
/// and get the data from AniList and embed it in a message.
pub async fn anilist_links_monitor(context: &Context, new_message: &Message) {
    if !should_embed(new_message.content.as_str()) {
        return;
    }

    let matches: Vec<_> = RE.captures_iter(new_message.content.as_str()).collect();
    let caps = &matches[0];
    match &caps[1] {
        "activity" => {}
        "character" => handle_character(&context, &new_message, &caps[2]).await,
        "studio" => {}
        "staff" => handle_staff(&context, &new_message, &caps[2]).await,
        _ => {}
    }
}

/// Handles activity embeds for the AniList Links Monitor
// fn handle_activity(context: &Context, message: &Message, activity_id: &str) {
//     let activity = client::search_activity(activity_id.into());
//
//     match_send!(context, message, activity, builders::activity_embed_builder);
// }

/// Handles character embeds for the AniList Links Monitor
async fn handle_character(context: &Context, message: &Message, character_id: &str) {
    let id: u64 = ok_or_return!(character_id.parse());
    let characters = vec![ok_or_return!(anilist::client::fetch_character(id).await)];

    let _ = AniListPagination::new_character_pagination(
        &context,
        &message,
        &characters,
        Default::default(),
    )
    .await;
}

/// Handles studio embeds for the AniList Links Monitor
// fn handle_studio(context: &Context, message: &Message, studio_id: &str) {
//     let studio: Option<Studio> = client::search_studio(studio_id.into());
//
//     match_send!(context, message, studio, builders::studio_embed_builder);
// }

/// Handles staff embeds for the AniList Links Monitor
async fn handle_staff(context: &Context, message: &Message, staff_id: &str) {
    let id: u64 = ok_or_return!(staff_id.parse());
    let staff = vec![ok_or_return!(anilist::client::fetch_staff(id).await)];

    let _ = AniListPagination::new_staff_pagination(&context, &message, &staff, Default::default())
        .await;
}
