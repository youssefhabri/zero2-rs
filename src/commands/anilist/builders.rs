use serenity::builder::CreateEmbed;
use crate::commands::anilist::models::{
    activity::Activity,
    character::Character,
    media::Media,
    user::User
};
use crate::commands::anilist::utils::synopsis;


pub fn media_pages_builder(results: Vec<Media>, embed_builder: fn(&Media, String) -> CreateEmbed) -> Vec<CreateEmbed> {
    let mut pages = vec![];
    let len = results.len().clone();

    for (i, media) in results.iter().enumerate() {
        pages.push(embed_builder(&media, format!("Page: {}/{} | ", i + 1, len)))
    }

    pages
}

pub fn user_pages_builder(results: Vec<User>, embed_builder: fn(&User, String) -> CreateEmbed) -> Vec<CreateEmbed> {
    let mut pages = vec![];
    let len = results.len().clone();

    for (i, user) in results.iter().enumerate() {
        pages.push(embed_builder(&user, format!("Page: {}/{} | ", i + 1, len)))
    }

    pages
}

pub fn character_pages_builder(results: Vec<Character>, embed_builder: fn(&Character, String) -> CreateEmbed) -> Vec<CreateEmbed> {
    let mut pages = vec![];
    let len = results.len().clone();

    for (i, character) in results.iter().enumerate() {
        pages.push(embed_builder(&character, format!("Page: {}/{} | ", i + 1, len)))
    }

    pages
}


pub fn anime_embed_builder(anime: &Media, prefix: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(3447003)
        .title(&anime.title.user_preferred)
        .url(&anime.site_url)
        .description(&anime.synopsis())
        .image(&anime.banner_image())
        .thumbnail(&anime.cover_image.large)
        .field("Score", &anime.mean_score(), true)
        .field("Episodes", &anime.episodes(), true)
        .field("Streaming Services", &anime.streaming_services(), true)
        .field("More info", &anime.tracking_sites(), true)
        .footer(|f| f
            .icon_url("https://anilist.co/img/icons/favicon-32x32.png")
            .text(format!("{}Status: {} | Powered by AniList", prefix, &anime.status())))
}

pub fn manga_embed_builder(manga: &Media, prefix: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(3447003)
        .title(&manga.title.user_preferred)
        .url(&manga.site_url)
        .description(&manga.synopsis())
        .image(&manga.banner_image())
        .thumbnail(&manga.cover_image.large)
        .field("Score", &manga.mean_score(), true)
        .field("Episodes", &manga.chapters(), true)
        .field("More info", &manga.tracking_sites(), true)
        .footer(|f| f
            .icon_url("https://anilist.co/img/icons/favicon-32x32.png")
            .text(format!("{}Status: {} | Powered by AniList", prefix, &manga.status())))
}

pub fn user_embed_builder(user: &User, prefix: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(3447003)
        .title(&user.name)
        .url(&user.site_url)
        .description(&user.about())
        .thumbnail(&user.avatar.large)
        .field("Watched time", &user.watched_time(), true)
        .field("Chapters read", &user.chapters_read(), true)
        .field("Favourite Anime", &user.favourite_anime(), true)
        .field("Favourite Manga", &user.favourite_manga(), true)
        .field("Favourite Characters", &user.favourite_characters(), true)
        .footer(|f| f
            .icon_url("https://anilist.co/img/icons/favicon-32x32.png")
            .text(format!("{}Powered by AniList", prefix)))
}

pub fn character_embed_builder(character: &Character, prefix: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(3447003)
        .title(&character.full_name())
        .url(&character.site_url)
        .description(&character.about())
        .thumbnail(&character.cover_image())
        .field("Anime", &character.media_list("ANIME"), true)
        .field("Manga", &character.media_list("MANGA"), true)
        .footer(|f| f
            .icon_url("https://anilist.co/img/icons/favicon-32x32.png")
            .text(format!("{}Powered by AniList", prefix)))
}

pub fn activity_embed_builder(activity: &Activity) -> CreateEmbed {
    let embed = match activity.__typename.as_str() {
        "TextActivity" => {
            text_activity_embed_builder(activity)
        },
        "ListActivity" => {
            list_activity_embed_builder(activity)
        },
        "MessageActivity" => {
            message_activity_embed_builder(activity)
        },
        _ => {
            CreateEmbed::default()
                .description("No activity was found.")
        }
    };

    embed
}

fn text_activity_embed_builder(activity: &Activity) -> CreateEmbed {
    let author = activity.user.clone().unwrap();
    CreateEmbed::default()
        .color(3447003)
        .title("Open activity in the browser")
        .url(&activity.site_url)
        .description(synopsis(&activity.text.clone().unwrap(), 1000))
        .author(|a| a
            .name(author.name.as_str())
            .url(author.site_url.as_str())
            .icon_url(author.avatar.large.as_str()))
        .footer(|f| f
            .icon_url("https://anilist.co/img/icons/favicon-32x32.png")
            .text("Powered by AniList"))
}

fn list_activity_embed_builder(activity: &Activity) -> CreateEmbed {
    let media = activity.media.clone().unwrap();
    let author = activity.user.clone().unwrap();
    CreateEmbed::default()
        .color(3447003)
        .description(format!("**{} [{}]({})**", activity.status().trim(), media.title.user_preferred, media.site_url))
        .thumbnail(&media.cover_image.large)
        .author(|a| a
            .name(author.name.as_str())
            .url(author.site_url.as_str())
            .icon_url(author.avatar.large.as_str()))
        .footer(|f| f
            .icon_url("https://anilist.co/img/icons/favicon-32x32.png")
            .text("Powered by AniList"))
}

fn message_activity_embed_builder(activity: &Activity) -> CreateEmbed {
    let author = activity.messenger.clone().unwrap();
    let recipient = activity.recipient.clone().unwrap();
    let message = synopsis(&activity.message.clone().unwrap(), 1000);
    CreateEmbed::default()
        .color(3447003)
        .title("Open activity in the browser")
        .url(&activity.site_url)
        .description(format!("**Sent a message to [{}]({})**\n\n{}", recipient.name, recipient.site_url, message))
        .author(|a| a
            .name(author.name.as_str())
            .url(author.site_url.as_str())
            .icon_url(author.avatar.large.as_str()))
        .footer(|f| f
            .icon_url("https://anilist.co/img/icons/favicon-32x32.png")
            .text("Powered by AniList"))
}