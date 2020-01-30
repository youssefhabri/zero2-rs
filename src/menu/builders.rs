use serenity::{builder::CreateEmbed, utils::Colour};
use urbandictionary::model::Definition;

use crate::commands::anilist::utils::synopsis;
use crate::models::anilist::{
    activity::Activity,
    character::Character,
    media::{Media, MediaType},
    staff::Staff,
    studio::Studio,
    user::User,
};
use crate::models::giphy::Giphy;

pub fn pages_builder<T>(
    results: Vec<T>,
    embed_builder: fn(&T, String) -> CreateEmbed,
) -> Vec<CreateEmbed> {
    results
        .iter()
        .enumerate()
        .map(|(i, item)| embed_builder(&item, format!("Page: {}/{} | ", i + 1, results.len())))
        .collect::<Vec<CreateEmbed>>()
}

pub fn media_embed_builder(media: &Media, prefix: String) -> CreateEmbed {
    let (field_name, value) = if media.media_type == MediaType::Anime {
        ("Episodes", media.episodes())
    } else {
        ("Chapters", media.chapters())
    };

    CreateEmbed::default()
        .color(3447003)
        .title(&media.title.user_preferred)
        .url(&media.site_url)
        .description(&media.synopsis())
        .image(&media.banner_image())
        .thumbnail(&media.cover_image.large)
        .field("Genres", &media.genres(), false)
        .field(field_name, value, true)
        .field("Score", &media.mean_score(), true)
        .field("More info", &media.tracking_sites(), true)
        .footer(|f| {
            f.icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                .text(format!(
                    "{}Status: {} | Powered by AniList",
                    prefix,
                    &media.status()
                ))
        })
        .clone()
}

pub fn user_embed_builder(user: &User, prefix: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(3447003)
        .title(&user.name)
        .url(&user.site_url)
        .description(&user.about())
        .thumbnail(&user.avatar.large)
        .field("Favourite Anime", &user.favourite_anime(), true)
        .field("Favourite Manga", &user.favourite_manga(), true)
        .field("Favourite Characters", &user.favourite_characters(), true)
        .field("Days Watched", &user.statistics.days_watched(), true)
        .field("Chapters read", &user.statistics.chapters_read(), true)
        .footer(|f| {
            f.icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                .text(format!("{}Powered by AniList", prefix))
        })
        .clone()
}

pub fn character_embed_builder(character: &Character, prefix: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(3447003)
        .title(&character.full_name())
        .url(&character.site_url)
        .description(&character.about())
        .thumbnail(&character.cover_image())
        .field("Anime", &character.media_list(MediaType::Anime), true)
        .field("Manga", &character.media_list(MediaType::Manga), true)
        .footer(|f| {
            f.icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                .text(format!("{}Powered by AniList", prefix))
        })
        .clone()
}

pub fn staff_embed_builder(staff: &Staff, prefix: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(3447003)
        .title(&staff.full_name())
        .url(&staff.site_url)
        .description(&staff.about())
        .thumbnail(&staff.image())
        .field("Anime", &staff.media_list(MediaType::Anime), true)
        .field("Manga", &staff.media_list(MediaType::Manga), true)
        .footer(|f| {
            f.icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                .text(format!("{}Powered by AniList", prefix))
        })
        .clone()
}

pub fn studio_embed_builder(studio: &Studio, prefix: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(3447003)
        .title(&studio.name())
        .url(&studio.site_url())
        .field("Anime", &studio.media(), false)
        .footer(|f| {
            f.icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                .text(format!("{}Powered by AniList", prefix))
        })
        .clone()
}

pub fn activity_embed_builder(activity: &Activity, _prefix: String) -> CreateEmbed {
    match activity.__typename.as_str() {
        "TextActivity" => text_activity_embed_builder(activity),
        "ListActivity" => list_activity_embed_builder(activity),
        "MessageActivity" => message_activity_embed_builder(activity),
        _ => CreateEmbed::default()
            .description("No activity was found.")
            .clone(),
    }
}

fn text_activity_embed_builder(activity: &Activity) -> CreateEmbed {
    let author = activity.user.clone().unwrap();
    CreateEmbed::default()
        .color(3447003)
        .title("Open activity in the browser")
        .url(&activity.site_url)
        .description(synopsis(&activity.text.clone().unwrap(), 1000))
        .author(|a| {
            a.name(author.name.as_str())
                .url(author.site_url.as_str())
                .icon_url(author.avatar.large.as_str())
        })
        .footer(|f| {
            f.icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                .text("Powered by AniList")
        })
        .clone()
}

fn list_activity_embed_builder(activity: &Activity) -> CreateEmbed {
    let media = activity.media.clone().unwrap();
    let author = activity.user.clone().unwrap();
    CreateEmbed::default()
        .color(3447003)
        .description(format!(
            "**{} [{}]({})**",
            activity.status().trim(),
            media.title.user_preferred,
            media.site_url
        ))
        .thumbnail(&media.cover_image.large)
        .author(|a| {
            a.name(author.name.as_str())
                .url(author.site_url.as_str())
                .icon_url(author.avatar.large.as_str())
        })
        .footer(|f| {
            f.icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                .text("Powered by AniList")
        })
        .clone()
}

fn message_activity_embed_builder(activity: &Activity) -> CreateEmbed {
    let author = activity.messenger.clone().unwrap();
    let recipient = activity.recipient.clone().unwrap();
    let message = synopsis(&activity.message.clone().unwrap(), 1000);
    CreateEmbed::default()
        .color(3447003)
        .title("Open activity in the browser")
        .url(&activity.site_url)
        .description(format!(
            "**Sent a message to [{}]({})**\n\n{}",
            recipient.name, recipient.site_url, message
        ))
        .author(|a| {
            a.name(author.name.as_str())
                .url(author.site_url.as_str())
                .icon_url(author.avatar.large.as_str())
        })
        .footer(|f| {
            f.icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                .text("Powered by AniList")
        })
        .clone()
}

// Giphy builders
pub fn giphy_pages_builder(
    results: Vec<Giphy>,
    embed_builder: fn(&Giphy, String) -> CreateEmbed,
) -> Vec<CreateEmbed> {
    let mut pages = vec![];
    let len = results.len();

    for (i, gif) in results.iter().enumerate() {
        pages.push(embed_builder(gif, format!("Page: {}/{} | ", i + 1, len)))
    }

    pages
}

pub fn giphy_embed_builder(gif: &Giphy, prefix: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(3447003)
        .title(&gif.title)
        .url(&gif.url)
        .image(&gif.images.original.url)
        .footer(|f| {
            f.icon_url("https://giphy.com/static/img/giphy_logo_square_social.png")
                .text(format!("{}Powered by Giphy", prefix))
        })
        .clone()
}

pub fn urban_embed_builder(definition: &Definition, prefix: String) -> CreateEmbed {
    let mut s = definition.definition.clone();
    if s.len() > 1800 {
        s.truncate(1800);
    }

    let mut embed = CreateEmbed::default();
    embed
        .color(Colour::GOLD)
        .title(&format!("Definition of {}", &definition.word))
        .url(&definition.permalink)
        .description(s)
        .footer(|f| f
            .icon_url("https://d2gatte9o95jao.cloudfront.net/assets/apple-touch-icon-1734beeaa059fbc5587bddb3001a0963670c6de8767afb6c67d88d856b0c0dad.png")
            .text(&format!("{}Defined by {}", prefix, definition.author)));

    // Only add example field if there's an example
    let example = definition.example.clone();
    if !example.is_empty() {
        embed.field("Example", example, true);
    }

    // This is a workaround since we can't order fields
    embed.field(
        "Votes",
        format!(
            "👍: **{}** 👎: **{}**",
            &definition.thumbs_up, &definition.thumbs_down
        ),
        true,
    );

    embed
}
