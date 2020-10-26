use serde::Deserialize;

use super::media::{MediaConnection, MediaEdge, MediaListStatus};
use super::shared::AniListID;
use crate::utils::{format_time, na_str, num_to_emoji, synopsis};

#[derive(Clone, Debug, Deserialize)]
pub struct UserAvatar {
    large: Option<String>,
    medium: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserFavourites {
    anime: MediaConnection,
    manga: MediaConnection,
    // characters: CharacterConnection,
    // staff: StaffConnection,
    // studio: StudioConnection,
}

impl UserFavourites {
    pub fn _favourites_from(&self, edges: Option<Vec<MediaEdge>>) -> String {
        let max = 10;

        if let Some(mut favourites) = edges {
            favourites.sort_by(|a, b| a.favourite_order.cmp(&b.favourite_order));
            let mut list = favourites
                .iter()
                .take(max)
                .map(|fav| fav.node.markdown_link())
                .collect::<Vec<_>>();

            if favourites.len() > max {
                list.push(format!("** + {} more ... **", favourites.len() - max));
            }

            if !list.is_empty() {
                return list.join("\n");
            }
        }

        na_str()
    }

    pub fn anime(&self) -> String {
        self._favourites_from(self.anime.edges.clone())
    }

    pub fn manga(&self) -> String {
        self._favourites_from(self.manga.edges.clone())
    }
}

#[derive(Clone, Debug, Deserialize)]
struct UserGenreStatistic {
    count: u32,
    genre: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserStatusStatistic {
    count: u32,
    status: MediaListStatus,
    minutes_watched: u32,
    chapters_read: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserAnimeStatistics {
    count: u32,
    mean_score: f32,
    standard_deviation: f32,
    minutes_watched: u32,
    episodes_watched: u32,
    genres: Vec<UserGenreStatistic>,
    statuses: Vec<UserStatusStatistic>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserMangaStatistics {
    count: u32,
    mean_score: f32,
    standard_deviation: f32,
    chapters_read: u32,
    genres: Vec<UserGenreStatistic>,
    statuses: Vec<UserStatusStatistic>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserStatisticsTypes {
    anime: UserAnimeStatistics,
    manga: UserMangaStatistics,
}

impl UserStatisticsTypes {
    fn media_count_by_status(statuses: &Vec<UserStatusStatistic>, status: MediaListStatus) -> u32 {
        statuses
            .iter()
            .find(|st| st.status == status)
            .map_or(0, |stat| stat.count)
    }

    pub fn total_anime(&self) -> u32 {
        self.anime.count
    }

    fn _anime_count_by_status(&self, status: MediaListStatus) -> u32 {
        Self::media_count_by_status(&self.anime.statuses, status)
    }

    pub fn anime_watching(&self) -> u32 {
        self._anime_count_by_status(MediaListStatus::Current)
    }

    pub fn anime_watched(&self) -> u32 {
        self._anime_count_by_status(MediaListStatus::Completed)
    }

    pub fn anime_planned(&self) -> u32 {
        self._anime_count_by_status(MediaListStatus::Planning)
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
            .find(|stat| stat.status == MediaListStatus::Planning)
            .map_or(0, |stat| stat.minutes_watched);

        format_time(f64::from(minutes_planned))
    }

    // TODO what about manga mean score?
    pub fn mean_score(&self) -> f32 {
        self.anime.mean_score
    }

    // TODO what about manga standard deviation?
    pub fn standard_deviation(&self) -> f32 {
        self.anime.standard_deviation
    }

    fn _manga_count_by_status(&self, status: MediaListStatus) -> u32 {
        Self::media_count_by_status(&self.manga.statuses, status)
    }

    pub fn manga_reading(&self) -> u32 {
        self._manga_count_by_status(MediaListStatus::Current)
    }

    pub fn manga_read(&self) -> u32 {
        self._manga_count_by_status(MediaListStatus::Completed)
    }

    pub fn manga_planned(&self) -> u32 {
        self._manga_count_by_status(MediaListStatus::Planning)
    }

    pub fn chapters_read(&self) -> String {
        format!("{}", self.manga.chapters_read)
    }

    fn _top_genres(&self, mut genres: Vec<UserGenreStatistic>) -> String {
        genres.sort_by(|a, b| b.count.cmp(&a.count));

        let result: String = genres
            .iter()
            .take(5)
            .enumerate()
            .map(|(i, g)| format!("{} {}", num_to_emoji((i + 1) as u32), g.genre))
            .collect::<Vec<String>>()
            .join("\n");

        if !result.is_empty() {
            return result;
        }

        na_str()
    }

    pub fn top_manga_genres(&self) -> String {
        self._top_genres(self.manga.genres.clone())
    }

    pub fn top_anime_genres(&self) -> String {
        self._top_genres(self.anime.genres.clone())
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserBase {
    pub id: u32,
    pub name: String,
    pub site_url: String,
    pub avatar: UserAvatar,
}

impl UserBase {
    pub fn avatar(&self) -> String {
        self.avatar
            .clone()
            .large
            .unwrap_or_else(User::default_avatar)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: AniListID,
    pub name: String,
    pub site_url: String,
    about: Option<String>,
    banner_image: Option<String>,
    pub avatar: UserAvatar,
    pub favourites: UserFavourites,
    pub statistics: UserStatisticsTypes,
}

impl User {
    pub fn about(&self) -> String {
        self.about
            .clone()
            .map_or_else(na_str, |desc| synopsis(desc, 300))
    }

    pub fn banner_image(&self) -> String {
        self.banner_image.clone().unwrap_or_default()
    }

    pub fn avatar(&self) -> String {
        self.avatar
            .clone()
            .large
            .unwrap_or_else(Self::default_avatar)
    }

    #[inline]
    pub fn default_avatar() -> String {
        "https://s4.anilist.co/file/anilistcdn/user/avatar/large/default.png".to_string()
    }
}
