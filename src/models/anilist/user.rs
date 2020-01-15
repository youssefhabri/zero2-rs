use crate::commands::anilist::utils::synopsis;
use crate::core::utils::format_time;
use crate::models::anilist::connection::{CharacterConnection, MediaConnection};
use crate::models::anilist::media::MediaType;

#[derive(Clone, Deserialize, Debug)]
pub struct UserAvatar {
    pub large: String,
}

#[derive(Deserialize, Debug)]
pub struct UserStatistics {
    pub anime: UserMediaStatistics,
    pub manga: UserMediaStatistics,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserMediaStatistics {
    count: u32,
    mean_score: f32,
    minutes_watched: u32,
    episodes_watched: u32,
    chapters_read: u32,
    volumes_read: u32,
    standard_deviation: f32,
    statuses: Vec<UserStatusStats>,
    genres: Vec<GenreStats>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserStatusStats {
    count: u32,
    mean_score: f32,
    chapters_read: u32,
    minutes_watched: u32,
    status: String,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenreStats {
    count: u32,
    genre: String,
    minutes_watched: u32,
    chapters_read: u32,
    mean_score: f32,
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

    pub statistics: UserStatistics,

    pub favourites: Favourites,
}

impl User {
    pub fn about(&self) -> String {
        match &self.about {
            Some(about) => synopsis(about, 300),
            None => String::new(),
        }
    }

    pub fn favourite_media(&self, fav_type: MediaType) -> String {
        let media_list = match fav_type {
            MediaType::Anime => &self.favourites.anime.nodes,
            MediaType::Manga => &self.favourites.manga.nodes,
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
        self.favourite_media(MediaType::Anime)
    }

    pub fn favourite_manga(&self) -> String {
        self.favourite_media(MediaType::Manga)
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

impl UserStatistics {
    pub fn total_anime(&self) -> u32 {
        self.anime.count
    }

    pub fn episodes_watched(&self) -> u32 {
        self.anime.episodes_watched
    }

    pub fn days_watched(&self) -> String {
        format_time(f64::from(self.anime.minutes_watched))
    }

    pub fn days_planned(&self) -> String {
        let minutes_planned = self
            .anime
            .statuses
            .iter()
            .find(|stat| stat.status.as_str() == "PLANNING")
            .map_or(0, |stat| stat.minutes_watched);

        format_time(f64::from(minutes_planned))
    }

    pub fn mean_score(&self) -> f32 {
        self.anime.mean_score
    }

    pub fn standard_deviation(&self) -> f32 {
        self.anime.standard_deviation
    }

    pub fn chapters_read(&self) -> String {
        format!("{}", self.manga.chapters_read)
    }
}
