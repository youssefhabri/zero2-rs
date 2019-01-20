use crate::commands::anilist::models::user::MediaConnection;

#[derive(Deserialize, Debug)]
pub struct CharacterName {
    first: Option<String>,
    last: Option<String>,
    native: Option<String>,
    alternative: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct CharacterImage {
    large: Option<String>,
    medium: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct CharacterBase {
    id: u32,

    #[serde(rename = "siteUrl")]
    site_url: String,

    name: CharacterName
}

#[derive(Deserialize, Debug)]
pub struct Character {
    id: u32,

    #[serde(rename = "siteUrl")]
    site_url: String,

    description: String,

    name: CharacterName,

    image: CharacterImage,

    media: MediaConnection
}