use serde::{de::Deserializer, Deserialize, Serialize};
use serde_json::Value;

use crate::commands::anilist::client::search_media_by_id;
use crate::models::anilist::media::Media;

fn ok_or_default<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Default,
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}

#[derive(Debug, Serialize)]
pub struct Body {
    pub image: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Source {
    pub from: f32,
    pub to: f32,
    pub at: f32,
    pub season: String,
    pub anilist_id: u64,
    pub mal_id: u64,
    pub filename: String,
    pub tokenthumb: String,
    pub anime: String,
    #[serde(deserialize_with = "ok_or_default")]
    pub episode: Option<u32>,
    pub similarity: f32,
    pub title: String,
    pub title_native: String,
    pub title_chinese: String,
    pub title_english: String,
    pub title_romaji: String,
    pub synonyms: Vec<String>,
    pub synonyms_chinese: Vec<String>,
    pub is_adult: bool,
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.anilist_id == other.anilist_id
    }
}

impl Source {
    pub fn anilist_url(&self) -> String {
        format!("https://anilist.co/anime/{}", self.anilist_id)
    }

    pub fn title(&self) -> String {
        self.title_romaji.clone()
    }

    pub fn image_preview(&self) -> String {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        format!(
            "https://trace.moe/thumbnail.php?anilist_id={}&file={}&t={}&token={}",
            self.anilist_id,
            utf8_percent_encode(&self.filename, NON_ALPHANUMERIC),
            self.at,
            self.tokenthumb
        )
    }

    pub fn similarity(&self) -> String {
        format!("{:02}%", (self.similarity * 100.0))
    }

    pub fn at(&self) -> String {
        let minutes = (self.at / 60.0) as u32;
        let seconds = (self.at % 60.0) as u32;
        format!("{}:{}", minutes, seconds)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceContainer {
    pub media: Media,
    pub source: Source,
}

impl SourceContainer {
    pub fn from_source(source: &Source) -> Self {
        let media = search_media_by_id(source.anilist_id.to_string(), "ANIME".to_string()).unwrap();

        SourceContainer {
            media,
            source: source.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "RawDocsCount")]
    raw_docs_count: u64,
    #[serde(rename = "RawDocsSearchTime")]
    raw_docs_search_time: u64,
    #[serde(rename = "ReRankSearchTime")]
    re_rank_search_time: u64,
    #[serde(rename = "CacheHit")]
    cache_hit: bool,
    trial: u32,
    limit: u32,
    limit_ttl: u32,
    quota: u32,
    quota_ttl: u32,
    pub docs: Vec<Source>,
}
