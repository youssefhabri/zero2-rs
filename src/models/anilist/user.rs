use crate::commands::anilist::utils::synopsis;
use crate::core::utils::format_time;
use crate::models::anilist::connection::{CharacterConnection, MediaConnection};

#[derive(Clone, Deserialize, Debug)]
pub struct UserAvatar {
    pub large: String,
}

#[derive(Deserialize, Debug)]
pub struct UserStats {
    #[serde(rename = "watchedTime")]
    pub watched_time: Option<u64>,

    #[serde(rename = "chaptersRead")]
    pub chapters_read: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Favourites {
    pub anime: MediaConnection,
    pub manga: MediaConnection,
    pub characters: CharacterConnection,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,

    #[serde(rename = "siteUrl")]
    pub site_url: String,

    pub avatar: UserAvatar,

    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,

    pub about: Option<String>,

    pub stats: UserStats,

    pub favourites: Favourites,
}

impl User {
    pub fn about(&self) -> String {
        match &self.about {
            Some(about) => synopsis(about, 300),
            None => String::new(),
        }
    }

    pub fn watched_time(&self) -> String {
        match &self.stats.watched_time {
            Some(watched_time) => format_time(*watched_time as f64),
            None => String::from("N/A"),
        }
    }

    pub fn chapters_read(&self) -> String {
        match &self.stats.chapters_read {
            Some(chapters_read) => format!("{}", chapters_read),
            None => String::from("N/A"),
        }
    }

    pub fn favourite_media(&self, fav_type: &str) -> String {
        let media_list = match fav_type {
            "ANIME" => &self.favourites.anime.nodes,
            "MANGA" => &self.favourites.manga.nodes,
            _ => return String::from("N/A"),
        };

        let mut fav_list: Vec<String> = vec![];

        if !media_list.is_empty() {
            for (i, media) in media_list.iter().enumerate() {
                if i == 5 {
                    break;
                }
                fav_list.push(format!(
                    "[{}]({})",
                    media.title.user_preferred, media.site_url
                ))
            }

            if media_list.len() > 5 {
                return format!("{}\n + {} more", fav_list.join("\n"), media_list.len() - 5);
            }

            return fav_list.join("\n");
        }

        String::from("N/A")
    }

    pub fn favourite_anime(&self) -> String {
        self.favourite_media("ANIME")
    }

    pub fn favourite_manga(&self) -> String {
        self.favourite_media("MANGA")
    }

    pub fn favourite_characters(&self) -> String {
        let character_list = &self.favourites.characters.nodes;

        let mut fav_list: Vec<String> = vec![];

        if !character_list.is_empty() {
            for (i, character) in character_list.iter().enumerate() {
                if i == 5 {
                    break;
                }
                fav_list.push(format!(
                    "[{}]({})",
                    character.full_name(),
                    character.site_url
                ))
            }

            if character_list.len() > 5 {
                return format!(
                    "{}\n + {} more",
                    fav_list.join("\n"),
                    character_list.len() - 5
                );
            }

            return fav_list.join("\n");
        }

        String::from("N/A")
    }
}
