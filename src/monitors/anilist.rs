use menu::anilist::AniListPagination;
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"https://anilist\.co/(character|activity|studio|staff)/([0-9]+)?/?([^/]+)?/?")
        .unwrap()
});

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
        "activity" => handle_activity(&context, &new_message, &caps[2]).await,
        "character" => handle_character(&context, &new_message, &caps[2]).await,
        "studio" => handle_studio(&context, &new_message, &caps[2]).await,
        "staff" => handle_staff(&context, &new_message, &caps[2]).await,
        _ => {}
    }
}

// Handles activity embeds for the AniList Links Monitor
async fn handle_activity(context: &Context, message: &Message, activity_id: &str) {
    let id: u64 = ok_or_return!(activity_id.parse());
    let activity = ok_or_return!(anilist::client::fetch_activity(id).await);
    let embed = menu::anilist::embeds::activity_embed(&activity);

    match_send!(&context, &message, &embed);
}

/// Handles character embeds for the AniList Links Monitor
async fn handle_character(context: &Context, message: &Message, character_id: &str) {
    let id: u64 = ok_or_return!(character_id.parse());
    let character = ok_or_return!(anilist::client::fetch_character(id).await);

    let _ = AniListPagination::new_character_pagination(
        &context,
        &message,
        &[character],
        Default::default(),
    )
    .await;
}

// Handles studio embeds for the AniList Links Monitor
async fn handle_studio(context: &Context, message: &Message, studio_id: &str) {
    let id: u64 = ok_or_return!(studio_id.parse());
    let studio = ok_or_return!(anilist::client::fetch_studio(id).await);
    let _ = AniListPagination::new_studio_pagination(&context, &message, &[studio]).await;
}

/// Handles staff embeds for the AniList Links Monitor
async fn handle_staff(context: &Context, message: &Message, staff_id: &str) {
    let id: u64 = ok_or_return!(staff_id.parse());
    let staff = ok_or_return!(anilist::client::fetch_staff(id).await);

    let _ =
        AniListPagination::new_staff_pagination(&context, &message, &[staff], Default::default())
            .await;
}
