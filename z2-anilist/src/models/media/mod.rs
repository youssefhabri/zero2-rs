use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

use super::character::CharacterRole;
use super::shared::{AniListID, FuzzyDate, PageInfo};
use crate::utils::{format_time, media_base_to_legend, na_long_str, na_str, synopsis};

mod enums;

pub use enums::{MediaFormat, MediaListStatus, MediaSeason, MediaSource, MediaStatus, MediaType};

pub type CountryCode = String;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: Option<String>,
    pub user_preferred: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MediaTrailer {
    id: String,
    site: String,
    thumbnail: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTag {
    id: AniListID,
    name: String,
    description: String,
    category: String,
    rank: u32,
    is_general_spoiler: bool,
    is_media_spoiler: bool,
    is_adult: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaCoverImage {
    color: Option<String>,
    medium: String,
    pub large: String,
    extra_large: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEdge {
    // id: AniListID,
    pub node: MediaBase,
    // relation_type: MediaRelation,
    // is_main_studio: bool,
    // characters: Vec<Character>,
    pub character_role: Option<CharacterRole>,
    // staff_role: String,
    // voice_actors: Vec<Staff>,
    pub favourite_order: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaConnection {
    pub edges: Option<Vec<MediaEdge>>,
    pub nodes: Option<Vec<MediaBase>>,
    page_info: Option<PageInfo>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringSchedule {
    id: AniListID,
    airing_at: u64,
    time_until_airing: u64,
    episode: u32,
    media_id: AniListID,
    media: Option<Box<Media>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recommendation {
    id: AniListID,
    rating: Option<i32>,
    media_recommendation: Option<MediaBase>,
    // user: User,
}

impl Recommendation {
    pub fn to_md_link(&self) -> Option<String> {
        if let Some(media) = &self.media_recommendation {
            let rating = self.rating.unwrap_or_default();
            let emoji = media.status.to_discord_emoji();
            let title = &media.title.user_preferred;
            let link = &media.site_url;

            return Some(format!("{} [{}]({}) [{}]", emoji, title, link, rating));
        }

        None
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationConnection {
    nodes: Vec<Recommendation>,
    page_info: Option<PageInfo>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MediaEternalLink {
    id: AniListID,
    url: String,
    site: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum MediaRankType {
    Rated,
    Popular,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaRank {
    id: AniListID,
    rank: u32,
    r#type: MediaRankType,
    format: MediaFormat,
    year: Option<u32>,
    season: Option<MediaSeason>,
    all_time: Option<bool>,
    context: String,
}

impl ToString for MediaRank {
    fn to_string(&self) -> String {
        let emoji = match self.r#type {
            MediaRankType::Popular => ":heart:",
            MediaRankType::Rated => ":star:",
        };
        let rank = self.rank;
        let context = &self.context;
        let season = self
            .season
            .clone()
            .map(|season| season.to_string())
            .unwrap_or_default();
        let year = self.year.map(|year| year.to_string()).unwrap_or_default();

        format!("{} {}# {} {} {}", emoji, rank, context, season, year)
            .trim()
            .to_string()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct StatusDistribution {
    amount: Option<u32>,
    status: Option<MediaListStatus>,
}

impl ToString for StatusDistribution {
    fn to_string(&self) -> String {
        if let Some(status) = &self.status {
            let score = self.amount.map_or_else(na_str, |score| score.to_string());

            return format!("**{}**: {}", status.to_string(), score);
        }

        na_str()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScoreDistribution {
    amount: Option<u32>,
    score: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaStats {
    status_distribution: Option<Vec<StatusDistribution>>,
    score_distribution: Option<Vec<ScoreDistribution>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaBase {
    id: AniListID,
    pub title: MediaTitle,
    pub status: MediaStatus,
    pub site_url: String,
    pub r#type: MediaType,
    pub cover_image: MediaCoverImage,
    episodes: Option<u32>,
    duration: Option<u32>,
    chapters: Option<u32>,
    average_score: Option<u32>,
}

impl MediaBase {
    pub fn markdown_link(&self) -> String {
        format!("[{}]({})", self.title.user_preferred, self.site_url)
    }

    pub fn markdown_link_with_status(&self) -> String {
        format!(
            "{} {}",
            self.status.to_discord_emoji(),
            self.markdown_link()
        )
    }

    pub fn markdown_link_with_status_and_score(&self) -> String {
        format!(
            "{} [Score: {}]",
            self.markdown_link_with_status(),
            self.average_score.map_or_else(na_str, |s| s.to_string())
        )
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: AniListID, // AniListID
    id_mal: Option<u32>,
    pub title: MediaTitle,
    pub r#type: MediaType,
    format: Option<MediaFormat>,
    status: Option<MediaStatus>,
    pub description: Option<String>,
    start_date: Option<FuzzyDate>,
    end_date: Option<FuzzyDate>,
    season: Option<MediaSeason>,
    season_year: Option<u32>,
    episodes: Option<u32>,
    duration: Option<u32>,
    chapters: Option<u32>,
    volumes: Option<u32>,
    country_of_origin: Option<CountryCode>,
    is_licensed: Option<bool>,
    source: Option<MediaSource>,
    hashtag: Option<String>,
    trailer: Option<MediaTrailer>,
    update_at: Option<u32>, // TODO This shouldn't be option. To be used for caching.
    pub cover_image: MediaCoverImage,
    banner_image: Option<String>,
    genres: Vec<String>,
    synonyms: Vec<String>,
    average_score: Option<u32>,
    mean_score: Option<u32>,
    popularity: Option<u32>,
    favourites: Option<u32>,
    rankings: Option<Vec<MediaRank>>,
    tags: Option<Vec<MediaTag>>,
    next_airing_episode: Option<AiringSchedule>,
    external_links: Option<Vec<MediaEternalLink>>,
    recommendations: Option<RecommendationConnection>,
    stats: Option<MediaStats>,
    pub site_url: String,
}

impl Media {
    pub fn synopsis(&self) -> String {
        self.description
            .clone()
            .map_or_else(na_str, |desc| synopsis(desc, 300))
    }

    pub fn average_score(&self) -> String {
        self.average_score
            .map_or_else(na_str, |average_score| average_score.to_string())
    }

    pub fn mean_score(&self) -> String {
        self.mean_score
            .map_or_else(na_str, |mean_score| mean_score.to_string())
    }

    pub fn popularity(&self) -> String {
        self.popularity
            .map_or_else(na_str, |popularity| popularity.to_string())
    }

    pub fn favourites(&self) -> String {
        self.favourites
            .map_or_else(na_str, |favourites| favourites.to_string())
    }

    pub fn rankings(&self) -> String {
        if let Some(rankings) = &self.rankings {
            let filter_rankings = |rank_type: MediaRankType| -> String {
                rankings
                    .iter()
                    .filter(|rank| rank.r#type == rank_type)
                    .map(|rank| rank.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            };

            let popular = filter_rankings(MediaRankType::Popular);
            let rated = filter_rankings(MediaRankType::Rated);

            let rankings = format!("{}\n\n{}", popular, rated).trim().to_string();
            if !rankings.is_empty() {
                return rankings;
            }
        }

        na_str()
    }

    pub fn status_distribution(&self) -> String {
        if let Some(stats) = &self.stats {
            if let Some(status_distribution) = &stats.status_distribution {
                let status_dist: String = status_distribution
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");

                if !status_dist.is_empty() {
                    return status_dist;
                }
            }
        }

        na_str()
    }

    pub fn episodes(&self) -> String {
        let total_episodes = match self.episodes {
            Some(total) => total.to_string(),
            None => "-".to_string(),
        };

        if self.is_releasing() {
            let current_airing_episode = match &self.next_airing_episode {
                Some(next_episode) => next_episode.episode.saturating_sub(1).to_string(),
                None => "-".to_string(),
            };

            return format!("{}/{}", current_airing_episode, total_episodes);
        }

        total_episodes
    }

    pub fn chapters(&self) -> String {
        self.chapters
            .map_or(na_str(), |chapters| chapters.to_string())
    }

    pub fn banner_image(&self) -> String {
        self.banner_image.clone().unwrap_or_default()
    }

    pub fn is_releasing(&self) -> bool {
        matches!(&self.status, Some(status) if *status == MediaStatus::Releasing)
    }

    pub fn genre_links(&self) -> String {
        if self.genres.is_empty() {
            return na_str();
        }

        let genre_url = |genre: &String| {
            format!(
                "[{}](https://anilist.co/search/anime?includedGenres={})",
                genre,
                genre.replace(" ", "+")
            )
        };

        self.genres
            .iter()
            .map(genre_url)
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn next_airing_episode(&self) -> String {
        let next_episode = self.next_airing_episode.clone().map(|next_episode| {
            let start = SystemTime::now();
            let since_unix_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let delta_time = next_episode.airing_at - since_unix_epoch.as_secs();

            format!(" | Next episode: {}", format_time((delta_time / 60) as f64))
        });

        format!("Airing{}", next_episode.unwrap_or_default())
    }

    pub fn status(&self) -> String {
        match &self.status {
            Some(status) => match status {
                MediaStatus::Releasing => match &self.r#type {
                    MediaType::Anime => self.next_airing_episode(),
                    MediaType::Manga => "Releasing".to_string(),
                },
                _ => status.to_string(),
            },
            None => "Unknown".to_string(),
        }
    }

    pub fn recommendations(&self) -> String {
        let compare_recommendations = |a: &Recommendation, b: &Recommendation| {
            b.rating
                .unwrap_or_default()
                .cmp(&a.rating.unwrap_or_default())
        };

        if let Some(recommendations_connection) = &self.recommendations {
            let max = 15;

            let mut recommendations = recommendations_connection.nodes.clone();
            recommendations.sort_by(compare_recommendations);

            let mut recs = recommendations
                .iter()
                .take(max)
                .filter_map(|rec| rec.to_md_link())
                .collect::<Vec<_>>();

            if let Some(page_info) = &recommendations_connection.page_info {
                if page_info.total > max {
                    let remaining = page_info.total - max;
                    recs.push(format!(" **+ {} more ...**", remaining));
                }
            }

            if !recs.is_empty() {
                return recs.join("\n");
            }
        }

        na_long_str()
    }

    pub fn recommendations_legend(&self) -> Option<String> {
        let recommendations = self.recommendations.as_ref()?;
        let media = recommendations
            .nodes
            .iter()
            .map(|recommendation| recommendation.media_recommendation.as_ref())
            .flatten()
            .cloned()
            .collect::<Vec<MediaBase>>();

        media_base_to_legend(&media)
    }
}
