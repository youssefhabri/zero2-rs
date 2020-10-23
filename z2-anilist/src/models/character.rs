use serde::Deserialize;

use crate::models::media::{MediaBase, MediaConnection};
use crate::models::{AniListID, MediaType};
use crate::utils::{na_long_str, synopsis};

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterConnection {
    edges: Vec<CharacterEdge>,
    // nodes: Vec<CharacterBase>,
    // page_info: PageInfo,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterEdge {
    node: CharacterBase,
    role: CharacterRole,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CharacterRole {
    Main,
    Supporting,
    Background,
}

impl ToString for CharacterRole {
    fn to_string(&self) -> String {
        match self {
            CharacterRole::Main => "Main",
            CharacterRole::Supporting => "Supporting",
            CharacterRole::Background => "Background",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterName {
    first: Option<String>,
    last: Option<String>,
    full: String,
    native: Option<String>,
    alternative: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterImage {
    large: Option<String>,
    medium: Option<String>,
}

type CharacterBase = Character;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: AniListID,
    name: CharacterName,
    image: CharacterImage,
    description: Option<String>,
    pub site_url: String,
    pub media: Option<MediaConnection>,
    favourites: u32,
}

impl Character {
    pub fn name(&self) -> String {
        self.name.full.clone()
    }

    pub fn synopsis(&self) -> String {
        if let Some(description) = &self.description {
            return synopsis(description, 300);
        }

        na_long_str()
    }

    pub fn avatar(&self) -> String {
        if let Some(image) = &self.image.large {
            return image.clone();
        }

        if let Some(image) = &self.image.medium {
            return image.clone();
        }

        "https://s4.anilist.co/file/anilistcdn/character/large/default.jpg".to_string()
    }

    fn _related_media_url(&self, media: &MediaBase, role: Option<CharacterRole>) -> String {
        let mut url = media.markdown_link_with_status();

        if let Some(role) = role {
            url.push_str(&format!(" ({} role)", role.to_string()));
        }

        url
    }

    fn _related_media(&self, r#type: MediaType) -> Option<String> {
        self.media
            .as_ref()
            .map(|media| {
                media.edges.as_ref().map(|edges| {
                    edges
                        .iter()
                        .filter(|edge| edge.node.r#type == r#type)
                        .map(|edge| {
                            self._related_media_url(&edge.node, edge.character_role.clone())
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                })
            })
            .flatten()
    }

    pub fn related_anime(&self) -> String {
        self._related_media(MediaType::Anime)
            .unwrap_or_else(na_long_str)
    }

    pub fn related_manga(&self) -> String {
        self._related_media(MediaType::Manga)
            .unwrap_or_else(na_long_str)
    }
}
