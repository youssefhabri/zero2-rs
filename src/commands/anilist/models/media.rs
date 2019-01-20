#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct MediaCoverImage {
    pub large: String,

    pub medium: String,
}

#[derive(Deserialize, Debug)]
pub struct MediaBase {
    pub id: u32,

    pub title: MediaTitle,

    #[serde(rename = "type")]
    pub media_type: String,

    #[serde(rename = "siteUrl")]
    pub site_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub id: u32,

    #[serde(rename = "idMal")]
    pub id_mal: u32,

    #[serde(rename = "type")]
    pub media_type: String,

    pub title: MediaTitle,

    #[serde(rename = "nextAiringEpisode")]
    pub next_airing_episode: Option<AiringSchedule>,

    pub status: String,

    #[serde(rename = "meanScore")]
    pub mean_score: u8,

    pub episodes: Option<u32>,

    pub chapters: Option<u32>,

    #[serde(rename = "siteUrl")]
    pub site_url: String,

    #[serde(rename = "externalLinks")]
    pub external_links: Vec<MediaExternalLink>,

    #[serde(rename = "coverImage")]
    pub cover_image: MediaCoverImage,

    #[serde(rename = "bannerImage")]
    pub banner_image: String,

    pub description: String,
}