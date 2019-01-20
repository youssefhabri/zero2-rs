use std::collections::HashMap;

pub mod user;
pub mod media;
pub mod character;


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
    #[serde(rename = "Page")]
    pub page: Page,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: Data,
}
