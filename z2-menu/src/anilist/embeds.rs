use anilist::models::{
    activity::ActivityUnion, user::UserBase, Activity, AiringSchedule, Character, Media, MediaType,
    Staff, Studio, User,
};
use chrono::{Datelike, NaiveDateTime, Utc, Weekday};
use serenity::builder::CreateEmbed;

const ANILIST_COLOR: u32 = 3447003;
const ANILIST_ICON: &str = "https://anilist.co/img/icons/favicon-32x32.png";

fn base_anilist_embed(footer: Option<String>) -> CreateEmbed {
    let footer = footer.unwrap_or_else(|| "Powered by AniList".to_string());

    CreateEmbed::default()
        .colour(ANILIST_COLOR)
        .footer(|f| f.icon_url(ANILIST_ICON).text(footer.to_string()))
        .to_owned()
}

// MEDIA EMBEDS
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
    let mut fields = Vec::new();

    if let Some(legend) = media.recommendations_legend() {
        fields.push(("Legend", legend, false));
    }

    base_anilist_embed(footer)
        .title(format!("{} - Recommendations", &media.title.user_preferred))
        .url(&media.site_url.to_string())
        .thumbnail(&media.cover_image.large)
        .description(media.recommendations())
        .fields(fields)
        .to_owned()
}

// USER EMBEDS
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

// CHARACTER EMBEDS
pub fn character_overview_embed(character: &Character, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(character.name())
        .url(&character.site_url)
        .description(character.synopsis())
        .thumbnail(character.avatar())
        .to_owned()
}

pub fn character_related_anime_embed(character: &Character, footer: Option<String>) -> CreateEmbed {
    let mut fields = Vec::new();

    if let Some(legend) = character.anime_legend() {
        fields.push(("Legend", legend, false));
    }

    base_anilist_embed(footer)
        .title(character.name())
        .url(&character.site_url)
        .thumbnail(character.avatar())
        .description(format!("**Related Anime**\n{}", character.related_anime()))
        .fields(fields)
        .to_owned()
}

pub fn character_related_manga_embed(character: &Character, footer: Option<String>) -> CreateEmbed {
    let mut fields = Vec::new();

    if let Some(legend) = character.manga_legend() {
        fields.push(("Legend", legend, false));
    }

    base_anilist_embed(footer)
        .title(character.name())
        .url(&character.site_url)
        .thumbnail(character.avatar())
        .description(format!("**Related Manga**\n{}", character.related_manga()))
        .fields(fields)
        .to_owned()
}

// STAFF EMBEDS
pub fn staff_overview_embed(staff: &Staff, footer: Option<String>) -> CreateEmbed {
    base_anilist_embed(footer)
        .title(staff.name())
        .url(&staff.site_url)
        .description(staff.synopsis())
        .thumbnail(staff.avatar())
        .to_owned()
}

pub fn staff_related_anime_embed(staff: &Staff, footer: Option<String>) -> CreateEmbed {
    let mut fields = Vec::new();

    if let Some(legend) = staff.anime_legend() {
        fields.push(("Legend", legend, false));
    }

    base_anilist_embed(footer)
        .title(staff.name())
        .url(&staff.site_url)
        .thumbnail(staff.avatar())
        .description(format!("**Related Anime**\n{}", staff.related_anime()))
        .fields(fields)
        .to_owned()
}

pub fn staff_related_manga_embed(staff: &Staff, footer: Option<String>) -> CreateEmbed {
    let mut fields = Vec::new();

    if let Some(legend) = staff.manga_legend() {
        fields.push(("Legend", legend, false));
    }

    base_anilist_embed(footer)
        .title(staff.name())
        .url(&staff.site_url)
        .thumbnail(staff.avatar())
        .description(format!("**Related Manga**\n{}", staff.related_manga()))
        .fields(fields)
        .to_owned()
}

// ACTIVITY EMBEDS
pub fn activity_embed(activity: &Activity) -> CreateEmbed {
    match activity.__typename {
        ActivityUnion::TextActivity => text_activity_embed(activity),
        ActivityUnion::ListActivity => list_activity_embed(activity),
        ActivityUnion::MessageActivity => message_activity_embed(activity),
    }
}

fn base_activity_embed(activity: &Activity, author: &UserBase) -> CreateEmbed {
    let datetime = NaiveDateTime::from_timestamp_opt(activity.created_at as i64, 0)
        .unwrap() // TODO: should check the option instead of unwrapping
        .format("%a, %B %e, %Y at %H:%M:%S")
        .to_string();
    let footer = Some(format!("Powered by AniList | {}", datetime));

    base_anilist_embed(footer)
        .description(activity.description())
        .author(|a| {
            a.name(author.name.as_str())
                .url(author.site_url.as_str())
                .icon_url(author.avatar())
        })
        .to_owned()
}

fn text_activity_embed(activity: &Activity) -> CreateEmbed {
    let author = activity.user.clone().unwrap();
    base_activity_embed(&activity, &author)
        .title("Open activity in the browser")
        .url(&activity.site_url)
        .to_owned()
}

fn list_activity_embed(activity: &Activity) -> CreateEmbed {
    let media = activity.media.clone().unwrap();
    let author = activity.user.clone().unwrap();
    base_activity_embed(&activity, &author)
        .thumbnail(&media.cover_image.large)
        .to_owned()
}

fn message_activity_embed(activity: &Activity) -> CreateEmbed {
    let author = activity.messenger.clone().unwrap();
    base_activity_embed(&activity, &author)
        .title("Open activity in the browser")
        .url(&activity.site_url)
        .to_owned()
}

// STUDIO EMBEDS
pub fn studio_embed(studio: &Studio, footer: Option<String>) -> CreateEmbed {
    let mut fields = vec![("Anime", studio.media(), false)];

    if let Some(legend) = studio.media_legend() {
        fields.push(("Legend", legend, false));
    }

    base_anilist_embed(footer)
        .title(&studio.name)
        .url(&studio.site_url)
        .fields(fields)
        .to_owned()
}

fn _weekday_to_string(weekday: Weekday) -> String {
    match weekday {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
    .to_string()
}

// TODO move those two functions somewhere else
fn _weekday_to_md(weekday: Weekday) -> String {
    let emoji = crate::utils::num_to_emoji(weekday as u32 + 1);
    let day = _weekday_to_string(weekday);
    let mut is_today = "";
    if weekday == Utc::now().weekday() {
        is_today = "**(Today)**";
    }

    format!("{} {} {}", emoji, day, is_today)
}

// AIRING SCHEDULE EMBEDS
pub fn airing_schedule_main_embed(footer: Option<String>) -> CreateEmbed {
    let weekdays = vec![
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];
    let days_md: Vec<String> = weekdays.into_iter().map(_weekday_to_md).collect();
    let description = format!(
        "Please select a Day to show the Airing Schedule for:\n{}",
        days_md.join("\n")
    );
    base_anilist_embed(footer)
        .description(description)
        .to_owned()
}

pub fn airing_schedule_embed(
    airing_schedule: &AiringSchedule,
    footer: Option<String>,
) -> CreateEmbed {
    let media = &airing_schedule.media;
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
}
