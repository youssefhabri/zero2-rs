use std::collections::HashMap;

use super::{Activity, AiringSchedule, Character, Media, Staff, Studio, User};

pub type Variables = HashMap<String, String>;

#[derive(Debug, Serialize)]
pub struct QueryBody {
    pub query: String,
    pub variables: Variables,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub airing_schedules: Option<Vec<AiringSchedule>>,
    pub characters: Option<Vec<Character>>,
    pub media: Option<Vec<Media>>,
    pub users: Option<Vec<User>>,
    pub staff: Option<Vec<Staff>>,
}

impl Default for Page {
    fn default() -> Self {
        Page {
            airing_schedules: None,
            characters: None,
            media: None,
            users: None,
            staff: None,
        }
    }
}

impl Page {
    pub fn airing_schedule(self) -> Vec<AiringSchedule> {
        match self.airing_schedules {
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

#[derive(Deserialize, Debug, Default)]
#[serde(default, rename_all = "PascalCase")]
pub struct Data {
    pub page: Page,
    pub activity: Option<Activity>,
    pub character: Option<Character>,
    pub media: Option<Media>,
    pub user: Option<User>,
    pub staff: Option<Staff>,
    pub studio: Option<Studio>,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: Data,
}
