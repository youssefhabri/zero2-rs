use crate::core::utils::format_time;
use crate::commands::anilist::utils::synopsis;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[derive(Clone, Deserialize, Debug)]
pub struct MediaTitle {
    pub romaji: Option<String>,

    pub english: Option<String>,

    pub native: Option<String>,

    #[serde(rename = "userPreferred")]
    pub user_preferred: String,
}

#[derive(Deserialize, Debug)]
pub struct AiringSchedule {
    #[serde(rename = "airingAt")]
    pub airing_at: u64,
}

#[derive(Deserialize, Debug)]
pub struct  MediaExternalLink {
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
    pub media_type: String,

    #[serde(rename = "siteUrl")]
    pub site_url: String,

    #[serde(rename = "coverImage")]
    pub cover_image: MediaCoverImage,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub id: u32,

    #[serde(rename = "idMal")]
    pub id_mal: Option<u32>,

    #[serde(rename = "type")]
    pub media_type: String,

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
            None => String::from("N/A")
        }
    }

    pub fn episodes(&self) -> String {
        match self.episodes {
            Some(episodes) => format!("{}", episodes),
            None => String::from("N/A")
        }
    }

    pub fn chapters(&self) -> String {
        match self.chapters {
            Some(chapters) => format!("{}", chapters),
            None => String::from("N/A")
        }
    }

    pub fn banner_image(&self) -> String {
        match &self.banner_image {
            Some(banner_image) => banner_image.to_string(),
            None => "".to_string()
        }
    }

    pub fn genres(&self) -> String {
        if !self.genres.is_empty() {
            let mut genres = vec![];
            let url = |genre: &String|
                format!("https://anilist.co/search/anime?includedGenres={}", genre.replace(" ", "+"));

            for genre in &self.genres {
                genres.push(format!("[{0}]({1})", genre, url(genre)));
            }

            return genres.join(", ")
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
        let anilist = format!("[AniList](https://anilist.co/anime/{})", self.id);
        let mal = match self.id_mal {
            Some(id_mal) => format!("[MyAnimeList](https://myanimelist.com/anime/{})", id_mal),
            None => "".to_owned()
        };

        format!("{}, {}", anilist, mal)
    }

    pub fn synopsis(&self) -> String {
        match &self.description {
            Some(description) => synopsis(description, 300),
            None => "".to_string()
        }
    }

    pub fn status(&self) -> String {
        let status = match self.status.as_str() {
            "FINISHED" => "Finished".to_string(),
            "RELEASING" => {
                if self.media_type == "ANIME" {
                    match &self.next_airing_episode {
                        Some(next) => {
                            let start = SystemTime::now();
                            let since_the_epoch = start.duration_since(UNIX_EPOCH)
                                .expect("Time went backwards");
                            let delta_time = next.airing_at - since_the_epoch.as_secs();

                            format!("Airing | Next episode: {}", format_time((delta_time / 60) as f64))
                        },
                        None => "Airing".to_string()
                    }
                } else {
                    "Releasing".to_string()
                }
            }
            "NOT_YET_RELEASED" => "Not Yet Released".to_string(),
            "CANCELLED" => "Cancelled".to_string(),
            _ => "Unknown Status".to_string()
        };

        status
    }
}