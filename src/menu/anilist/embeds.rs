use anilist::models::{Character, Media, MediaType, Staff, User};
use serenity::builder::CreateEmbed;

const ANILIST_COLOR: u32 = 3447003;
const ANILIST_ICON: &str = "https://anilist.co/img/icons/favicon-32x32.png";

fn base_anilist_embed(footer: Option<String>) -> CreateEmbed {
    let footer = footer.unwrap_or_else(|| "Powered by AniList".to_string());

    CreateEmbed::default()
        .colour(ANILIST_COLOR)
        .footer(|f| f.icon_url(ANILIST_ICON).text(format!("{}", footer)))
        .to_owned()
}

/// MEDIA EMBEDS
pub fn media_overview_embed(media: &Media, footer: Option<String>) -> CreateEmbed {
    let (field_name, value) = match &media.r#type {
        MediaType::Anime => ("Episodes", media.episodes()),
        MediaType::Manga => ("Chapters", media.chapters()),
    };

    base_anilist_embed(footer)
        .title(&media.title.user_preferred)
        .url(&media.site_url)
        .description(&media.synopsis())
        .image(&media.banner_image())
        .thumbnail(&media.cover_image.large)
        .field(field_name, value, true)
        .field("Score", &media.mean_score(), true)
        .field("Genres", media.genre_links(), true)
        .to_owned()
    // .field("More info", &media.tracking_sites(), true)
}

pub fn media_stats_embed(media: &Media, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(format!("{} - Stats", &media.title.user_preferred))
        .url(format!("{}/stats", &media.site_url))
        .thumbnail(&media.cover_image.large)
        .field("Average Score", media.average_score(), true)
        .field("Mean Score", media.mean_score(), true)
        .field("Popularity", media.popularity(), true)
        .field("Favourites", media.favourites(), true)
        .field("Rankings", media.rankings(), false)
        .field("Status Distributions", media.status_distribution(), false)
        .to_owned()
}

pub fn media_recommendations_embed(media: &Media, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(format!("{} - Recommendations", &media.title.user_preferred))
        .url(format!("{}", &media.site_url))
        .thumbnail(&media.cover_image.large)
        .description(media.recommendations())
        .field("Legend", "**Finished:** :white_small_square:  - **Releasing:** :small_blue_diamond: - **Not Yet Released:** :small_orange_diamond: - **Cancelled:** :black_small_square:", false)
        .to_owned()
}

/// USER EMBEDS
pub fn user_overview_embed(user: &User, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(&user.name)
        .url(&user.site_url)
        .description(&user.about())
        .thumbnail(&user.avatar())
        .field("Days Watched", user.statistics.days_watched(), true)
        .field("Chapters Read", user.statistics.chapters_read(), true)
        .to_owned()
}

pub fn user_stats_embed(user: &User, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(format!("{} - Stats", &user.name))
        .url(format!("{}/stats", user.site_url))
        .field("Currently Watching", user.statistics.anime_watching(), true)
        .field("Anime Watched", user.statistics.anime_watched(), true)
        .field("Anime Planned", user.statistics.anime_planned(), true)
        .field("Currently Reading", user.statistics.manga_reading(), true)
        .field("Manga Read", user.statistics.manga_read(), true)
        .field("Manga Planned", user.statistics.manga_planned(), true)
        .field(
            "Top 5 Anime Genres",
            user.statistics.top_anime_genres(),
            true,
        )
        .field(
            "Top 5 Manga Genres",
            user.statistics.top_manga_genres(),
            true,
        )
        .to_owned()
}

pub fn user_favourites_embed(user: &User, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(&user.name)
        .url(&user.site_url)
        .field("Favourite Anime", user.favourites.anime(), true)
        .field("Favourite Manga", user.favourites.manga(), true)
        .to_owned()
}

/// CHARACTER EMBEDS
pub fn character_overview_embed(character: &Character, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(character.name())
        .url(&character.site_url)
        .description(character.synopsis())
        .thumbnail(character.avatar())
        .to_owned()
}

pub fn character_related_anime_embed(character: &Character, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(character.name())
        .url(&character.site_url)
        .thumbnail(character.avatar())
        .field("Related Anime", character.related_anime(), true)
        .field("Legend", "**Finished:** :white_small_square:  - **Releasing:** :small_blue_diamond: - **Not Yet Released:** :small_orange_diamond: - **Cancelled:** :black_small_square:", false)
        .to_owned()
}

pub fn character_related_manga_embed(character: &Character, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(character.name())
        .url(&character.site_url)
        .thumbnail(character.avatar())
        .field("Related Manga", character.related_manga(), true)
        .field("Legend", "**Finished:** :white_small_square:  - **Releasing:** :small_blue_diamond: - **Not Yet Released:** :small_orange_diamond: - **Cancelled:** :black_small_square:", false)
        .to_owned()
}

/// STAFF EMBEDS

pub fn staff_overview_embed(staff: &Staff, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(staff.name())
        .url(&staff.site_url)
        .description(staff.synopsis())
        .thumbnail(staff.avatar())
        .to_owned()
}

pub fn staff_related_anime_embed(staff: &Staff, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(staff.name())
        .url(&staff.site_url)
        .thumbnail(staff.avatar())
        .field("Related Anime", staff.related_anime(), true)
        .field("Legend", "**Finished:** :white_small_square:  - **Releasing:** :small_blue_diamond: - **Not Yet Released:** :small_orange_diamond: - **Cancelled:** :black_small_square:", false)
        .to_owned()
}

pub fn staff_related_manga_embed(staff: &Staff, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(staff.name())
        .url(&staff.site_url)
        .thumbnail(staff.avatar())
        .field("Related Manga", staff.related_manga(), true)
        .field("Legend", "**Finished:** :white_small_square:  - **Releasing:** :small_blue_diamond: - **Not Yet Released:** :small_orange_diamond: - **Cancelled:** :black_small_square:", false)
        .to_owned()
}
