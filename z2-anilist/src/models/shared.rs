use serde::Deserialize;

use super::{Character, Media, Staff, User};

pub type AniListID = u64;

#[derive(Clone, Debug, Deserialize)]
pub struct FuzzyDate {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub total: usize,
    per_page: usize,
    current_page: usize,
    last_page: usize,
    has_next_page: bool,
}

#[derive(Debug, Deserialize)]
pub struct Single<T> {
    pub item: Option<T>,
}

#[derive(Debug, Deserialize)]
pub struct MultiItemContainer<T> {
    pub items: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct Paged<T> {
    #[serde(rename = "Page")]
    pub page: MultiItemContainer<T>,
}

#[derive(Debug, Deserialize)]
pub struct AniListError {
    message: Option<String>,
    status: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AniListResponse<T> {
    pub data: T,
    pub errors: Option<Vec<AniListError>>,
}

make_response!(MediaResponse, Media, media);
make_paged_response!(PagedMediaResponse, Media, media);

make_response!(UserResponse, User, user);
make_paged_response!(PagedUserResponse, User, users);

make_response!(CharacterResponse, Character, character);
make_paged_response!(PagedCharacterResponse, Character, characters);

make_response!(StaffResponse, Staff, staff);
make_paged_response!(PagedStaffResponse, Staff, staff);
