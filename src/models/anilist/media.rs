use crate::commands::anilist::utils::synopsis;
use crate::core::utils::format_time;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum MediaType {
    Anime,
    Manga,
}

impl ToString for MediaType {
    fn to_string(&self) -> String {
        match self {
            MediaType::Anime => "ANIME",
            MediaType::Manga => "MANGA",
        }
        .to_string()
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct MediaTitle {
    pub romaji: Option<String>,

    pub english: Option<String>,

    pub native: Option<String>,

    #[serde(rename = "userPreferred")]
    pub user_preferred: String,
}

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct AiringSchedule {
    #[serde(rename = "airingAt")]
    pub airing_at: u64,
    pub episode: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct MediaExternalLink {
    pub url: String,

    pub site: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MediaCoverImage {
    pub large: String,

    pub medium: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MediaBase {
    pub id: u32,

    pub title: MediaTitle,

    #[serde(rename = "type")]
    pub media_type: MediaType,

    #[serde(rename = "siteUrl")]
    pub site_url: String,

    #[serde(rename = "coverImage")]
    pub cover_image: MediaCoverImage,

    #[serde(rename = "averageScore")]
    pub average_score: Option<u32>,

    #[serde(rename = "meanScore")]
    pub mean_score: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub id: u32,

    #[serde(rename = "idMal")]
    pub id_mal: Option<u32>,

    #[serde(rename = "type")]
    pub media_type: MediaType,

    pub title: MediaTitle,

    #[serde(rename = "nextAiringEpisode")]
    pub next_airing_episode: Option<AiringSchedule>,

    pub status: String,

    #[serde(rename = "isAdult")]
    pub is_adult: bool,

    #[serde(rename = "meanScore")]
    pub mean_score: Option<u8>,

    pub episodes: Option<u32>,

    pub chapters: Option<u32>,

    #[serde(rename = "siteUrl")]
    pub site_url: String,

    #[serde(rename = "externalLinks")]
    pub external_links: Vec<MediaExternalLink>,

    #[serde(rename = "coverImage")]
    pub cover_image: MediaCoverImage,

    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,

    pub description: Option<String>,

    genres: Vec<String>,
}

impl Media {
    pub fn mean_score(&self) -> String {
        match self.mean_score {
            Some(score) => format!("{}", score),
            None => String::from("N/A"),
        }
    }

    pub fn episodes(&self) -> String {
        let total_episodes = match self.episodes {
            Some(total) => total.to_string(),
            None => "-".to_string(),
        };
        let current_airing_episode = match self.next_airing_episode() {
            Some(episode) => (episode.saturating_sub(1)).to_string(),
            None => "-".to_string(),
        };

        match self.is_releasing() {
            true => format!("{}/{}", current_airing_episode, total_episodes),
            false => total_episodes,
        }
    }

    fn next_airing_episode(&self) -> Option<u32> {
        if dbg!(self.next_airing_episode).is_some() {
            return dbg!(self.next_airing_episode.unwrap().episode);
        }

        None
    }

    pub fn chapters(&self) -> String {
        match self.chapters {
            Some(chapters) => format!("{}", chapters),
            None => String::from("N/A"),
        }
    }

    pub fn banner_image(&self) -> String {
        match &self.banner_image {
            Some(banner_image) => banner_image.to_string(),
            None => "".to_string(),
        }
    }

    pub fn genres(&self) -> String {
        if !self.genres.is_empty() {
            let mut genres = vec![];
            let url = |genre: &String| {
                format!(
                    "https://anilist.co/search/anime?includedGenres={}",
                    genre.replace(" ", "+")
                )
            };

            for genre in &self.genres {
                genres.push(format!("[{0}]({1})", genre, url(genre)));
            }

            return genres.join(", ");
        }

        "N/A".to_string()
    }

    pub fn _streaming_services(&self) -> String {
        if !self.external_links.is_empty() {
            let mut list: Vec<String> = vec![];
            for service in &self.external_links {
                list.push(format!("[{}]({})", service.site, service.url));
            }
            return list.join(", ");
        }

        "Not available".to_string()
    }

    pub fn tracking_sites(&self) -> String {
        let mut sites = vec![];
        sites.push(format!("[AniList]({})", self.site_url));
        if let Some(id) = self.id_mal {
            sites.push(format!(
                "[MyAnimeList](https://myanimelist.com/anime/{})",
                id
            ));
        }

        sites.join(", ")
    }

    pub fn synopsis(&self) -> String {
        match &self.description {
            Some(description) => synopsis(description, 300),
            None => "".to_string(),
        }
    }

    pub fn status(&self) -> String {
        match self.status.as_str() {
            "FINISHED" => "Finished".to_string(),
            "RELEASING" => {
                if self.media_type == MediaType::Anime {
                    match &self.next_airing_episode {
                        Some(next) => {
                            let start = SystemTime::now();
                            let since_the_epoch = start
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards");
                            let delta_time = next.airing_at - since_the_epoch.as_secs();

                            format!(
                                "Airing | Next episode: {}",
                                format_time((delta_time / 60) as f64)
                            )
                        }
                        None => "Airing".to_string(),
                    }
                } else {
                    "Releasing".to_string()
                }
            }
            "NOT_YET_RELEASED" => "Not Yet Released".to_string(),
            "CANCELLED" => "Cancelled".to_string(),
            _ => "Unknown Status".to_string(),
        }
    }

    pub fn is_releasing(&self) -> bool {
        self.status == "RELEASING"
    }
}
