use serde::Deserialize;

use super::media::MediaConnection;
use super::AniListID;
use crate::utils::{media_base_to_legend, na_long_str};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Studio {
    id: AniListID,
    pub name: String,
    pub site_url: String,
    media: MediaConnection,
    favourites: u32,
}

impl Studio {
    pub fn media(&self) -> String {
        self.media
            .nodes
            .clone()
            .map(|media| {
                media
                    .iter()
                    .take(10)
                    .map(|media| media.markdown_link_with_status_and_score())
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .filter(|media| !media.is_empty())
            .unwrap_or_else(na_long_str)
    }

    pub fn media_legend(&self) -> Option<String> {
        let media = self.media.nodes.as_ref()?;
        media_base_to_legend(&media)
    }
}
