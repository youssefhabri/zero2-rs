use super::MediaBase;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringSchedule {
    id: u32,
    time_until_airing: Option<i64>,
    episode: Option<u32>,
    media: MediaBase,
}

impl AiringSchedule {
    pub fn to_url(&self) -> String {
        let episode = match &self.episode {
            Some(episode) => format!("Ep. {}", episode),
            None => String::from("Ep. N/A"),
        };

        format!(
            "[{0}] [{1}]({2})",
            episode, &self.media.title.user_preferred, &self.media.site_url
        )
    }
}
