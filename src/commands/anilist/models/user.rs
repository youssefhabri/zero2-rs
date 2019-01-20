use crate::commands::anilist::models::{
    media::MediaBase,
    character::CharacterBase,
};

#[derive(Deserialize, Debug)]
pub struct UserAvatar {
    large: String,
}

#[derive(Deserialize, Debug)]
pub struct UserStats {
    #[serde(rename = "watchedTime")]
    watched_time: Option<u32>,

    #[serde(rename = "chaptersRead")]
    chapters_read: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct MediaConnection {
    nodes: Vec<MediaBase>
}

#[derive(Deserialize, Debug)]
pub struct CharacterConnection {
    nodes: Vec<CharacterBase>
}

#[derive(Deserialize, Debug)]
pub struct Favourites {
    anime: MediaConnection,
    manga: MediaConnection,
    characters: CharacterConnection
}

#[derive(Deserialize, Debug)]
pub struct User {
    id: u32,
    name: String,

    #[serde(rename = "siteUrl")]
    site_url: String,

    avatar: UserAvatar,

    #[serde(rename = "bannerImage")]
    banner_image: Option<String>,

    about: Option<String>,

    stats: UserStats,

    favourites: Favourites
}