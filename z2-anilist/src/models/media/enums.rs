use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum MediaType {
    Anime,
    Manga,
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            MediaType::Anime => "ANIME",
            MediaType::Manga => "MANGA",
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaFormat {
    Tv,
    TvShort,
    Movie,
    Special,
    Ova,
    Ona,
    Music,
    Manga,
    Novel,
    OneShot,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaStatus {
    Finished,
    Releasing,
    NotYetReleased,
    Cancelled,
}

impl MediaStatus {
    pub fn to_discord_emoji(&self) -> String {
        match self {
            MediaStatus::Finished => ":white_small_square:",
            MediaStatus::Releasing => ":small_blue_diamond:",
            MediaStatus::NotYetReleased => ":small_orange_diamond:",
            MediaStatus::Cancelled => ":black_small_square:",
        }
        .to_string()
    }

    pub fn to_string_with_emoji(&self) -> String {
        format!("{}: {}", self.to_string(), self.to_discord_emoji())
    }
}

impl std::fmt::Display for MediaStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            MediaStatus::Finished => "Finished",
            MediaStatus::Releasing => "Releasing",
            MediaStatus::NotYetReleased => "Not Yet Released",
            MediaStatus::Cancelled => "Cancelled",
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum MediaListStatus {
    Current,
    Planning,
    Completed,
    Dropped,
    Paused,
    Repeating,
}

impl std::fmt::Display for MediaListStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            MediaListStatus::Current => "Current",
            MediaListStatus::Planning => "Planning",
            MediaListStatus::Completed => "Completed",
            MediaListStatus::Dropped => "Dropped",
            MediaListStatus::Paused => "Paused",
            MediaListStatus::Repeating => "Repeating",
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaSeason {
    Winter,
    Spring,
    Summer,
    Fall,
}

impl std::fmt::Display for MediaSeason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            MediaSeason::Winter => "Winter",
            MediaSeason::Spring => "Spring",
            MediaSeason::Summer => "Summer",
            MediaSeason::Fall => "Fall",
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaSource {
    Anime,
    Doujinshi,
    LightNovel,
    Manga,
    Novel,
    Other,
    Original,
    VideoGame,
    VisualNovel,
}
