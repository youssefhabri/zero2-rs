use std::collections::HashMap;

use super::{Activity, AiringSchedule, Character, Media, Staff, Studio, User};

pub type Variables = HashMap<String, String>;

#[derive(Debug, Serialize)]
pub struct QueryBody {
    pub query: String,
    pub variables: Variables,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    #[serde(rename = "airingSchedules")]
    pub airing_schedule: Option<Vec<AiringSchedule>>,

    pub characters: Option<Vec<Character>>,

    pub media: Option<Vec<Media>>,

    pub users: Option<Vec<User>>,

    pub staff: Option<Vec<Staff>>,
}

impl Default for Page {
    fn default() -> Self {
        Page {
            airing_schedule: None,
            characters: None,
            media: None,
            users: None,
            staff: None,
        }
    }
}

impl Page {
    pub fn airing_schedule(self) -> Vec<AiringSchedule> {
        match self.airing_schedule {
            Some(airing_schedule) => airing_schedule,
            None => vec![],
        }
    }

    pub fn media(self) -> Vec<Media> {
        match self.media {
            Some(media) => media,
            None => vec![],
        }
    }

    pub fn users(self) -> Vec<User> {
        match self.users {
            Some(user) => user,
            None => vec![],
        }
    }

    pub fn characters(self) -> Vec<Character> {
        match self.characters {
            Some(characters) => characters,
            None => vec![],
        }
    }

    pub fn staff(self) -> Vec<Staff> {
        match self.staff {
            Some(staff) => staff,
            None => vec![],
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(default, rename = "Page")]
    pub page: Page,

    #[serde(default, rename = "Activity")]
    pub activity: Option<Activity>,

    #[serde(default, rename = "Character")]
    pub character: Option<Character>,

    #[serde(default, rename = "Media")]
    pub media: Option<Media>,

    #[serde(default, rename = "User")]
    pub user: Option<User>,

    #[serde(default, rename = "Staff")]
    pub staff: Option<Staff>,

    #[serde(default, rename = "Studio")]
    pub studio: Option<Studio>,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: Data,
}
