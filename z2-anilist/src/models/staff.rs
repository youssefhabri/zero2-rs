use serde::Deserialize;

use crate::models::character::{CharacterConnection, CharacterRole};
use crate::models::media::{MediaBase, MediaConnection, MediaType};
use crate::models::AniListID;
use crate::utils::{na_long_str, synopsis};

/// The names of the staff member
#[derive(Clone, Debug, Deserialize)]
pub struct StaffName {
    /// The person's given name
    first: Option<String>,

    /// The person's surname
    last: Option<String>,

    /// The person's full name
    full: String,

    /// The person's full name in their native language
    native: Option<String>,

    /// Other names the staff member might be referred to as (pen names)
    alternative: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StaffImage {
    large: Option<String>,
    medium: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Staff {
    /// The id of the staff memeber
    pub id: AniListID,

    /// The names of the staff member
    name: StaffName,

    //language: StaffLanguage,
    /// The staff image
    image: StaffImage,

    /// A general description of the staff member
    description: Option<String>,

    /// The url for the staff page on AniList website
    pub site_url: String,

    /// `Media` where the staff member has a production role
    staff_media: MediaConnection,

    /// Characters voiced by the actor
    characters: CharacterConnection,

    /// The amount of user's who have favourited the staff member
    favourites: u32,
}

impl Staff {
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

        "https://s4.anilist.co/file/anilistcdn/staff/large/default.jpg".to_string()
    }

    fn _related_media_url(&self, media: &MediaBase, role: Option<CharacterRole>) -> String {
        let mut url = media.markdown_link_with_status();

        if let Some(role) = role {
            url.push_str(&format!(" ({} role)", role.to_string()));
        }

        url
    }

    fn _related_media(&self, r#type: MediaType) -> Option<String> {
        self.staff_media
            .edges
            .as_ref()
            .map(|edges| {
                edges
                    .iter()
                    .filter(|edge| edge.node.r#type == r#type)
                    .map(|edge| self._related_media_url(&edge.node, edge.character_role.clone()))
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .filter(|media| !media.is_empty())
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
