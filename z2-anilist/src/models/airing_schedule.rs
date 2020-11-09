use serde::Deserialize;

use crate::models::media::Media;
use crate::models::AniListID;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringSchedule {
    /// The time the episode airs at
    airing_at: i64,
    /// The airing episode number
    episode: u32,
    /// The id of the airing schedule item
    pub id: AniListID,
    /// The associate media of the airing episode
    pub media: Media,
    /// The associate media id of the airing episode
    media_id: AniListID,
    /// Seconds until episode starts airing
    time_until_airing: i64,
}
