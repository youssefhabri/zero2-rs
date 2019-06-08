use crate::models::anilist::character::CharacterBase;
use crate::models::anilist::media::MediaBase;

#[derive(Clone, Deserialize, Debug)]
pub struct MediaConnection {
    pub nodes: Vec<MediaBase>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CharacterConnection {
    pub nodes: Vec<CharacterBase>,
}
