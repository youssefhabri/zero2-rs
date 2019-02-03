use std::collections::HashMap;
use crate::commands::anilist::models::activity::Activity;

pub mod user;
pub mod media;
pub mod character;
pub mod activity;


pub type Variables = HashMap<String, String>;

#[derive(Serialize)]
pub struct QueryBody {
    pub query: String,
    pub variables: Variables,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    pub media: Option<Vec<media::Media>>,
    pub users: Option<Vec<user::User>>,
    pub characters: Option<Vec<character::Character>>,
}

impl Default for Page {
    fn default() -> Self {
        Page {
            media: None,
            users: None,
            characters: None,
        }
    }
}

impl Page {
    pub fn media(self) -> Vec<media::Media> {
        match self.media {
            Some(media) => media,
            None => vec![]
        }
    }

    pub fn users(self) -> Vec<user::User> {
        match self.users {
            Some(user) => user,
            None => vec![]
        }
    }

    pub fn characters(self) -> Vec<character::Character> {
        match self.characters {
            Some(character) => character,
            None => vec![]
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(default, rename = "Page")]
    pub page: Page,

    #[serde(default, rename = "Activity")]
    pub activity: Option<Activity>
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: Data,
}
