use crate::models::anilist::media::MediaBase;
use crate::models::anilist::character::CharacterBase;

#[derive(Deserialize, Debug)]
pub struct MediaConnection {
    pub nodes: Vec<MediaBase>
}

#[derive(Deserialize, Debug)]
pub struct CharacterConnection {
    pub nodes: Vec<CharacterBase>
}
