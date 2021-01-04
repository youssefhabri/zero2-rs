use reqwest::{Client as ReqwestClient, Error as ReqwestError};
use rust_embed::RustEmbed;
use serde::{de::DeserializeOwned, Serialize};
use std::result::Result as StdResult;
use thiserror::Error;

use crate::models::shared::{
    ActivityResponse, AiringScheduleResponse, AniListError, AniListID, CharacterResponse,
    MediaResponse, PagedAiringScheduleResponse, PagedCharacterResponse, PagedMediaResponse,
    PagedStaffResponse, PagedStudioResponse, PagedUserResponse, StaffResponse, StudioResponse,
    UserResponse,
};
use crate::models::{Activity, AiringSchedule, Character, Media, MediaType, Staff, Studio, User};
use crate::query_variables::{
    AiringScheduleVariables, MediaVariables, StandardVariables, Variables,
};

const API_URL: &str = "https://graphql.anilist.co";

#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] ReqwestError),

    #[error("AniList Activity (id: {0}) not found")]
    ActivityNotFound(AniListID),

    #[error("AniList AiringSchedule (id: {0}) not found")]
    AiringScheduleNotFound(AniListID),

    #[error("AniList Media (id: {0}) not found")]
    MediaNotFound(AniListID),

    #[error("AniList User (id: {0}) not found")]
    UserNotFound(AniListID),

    #[error("AniList Character (id: {0}) not found")]
    CharacterNotFound(AniListID),

    #[error("AniList Staff (id: {0}) not found")]
    StaffNotFound(AniListID),

    #[error("AniList Studio (id: {0}) not found")]
    StudioNotFound(AniListID),

    #[error("AniList returned some errors: {0:?}")]
    AniListErrors(Vec<AniListError>),

    #[error("Error loading the graphql file")]
    GraphQLLoadError,
}

type AniListResult<T> = StdResult<T, Error>;

#[derive(Serialize)]
pub struct QueryBody<'a> {
    pub query: String,
    pub variables: Box<dyn Variables + 'a>,
}

enum GQLFile<'a> {
    Query(&'a str),
    Fragment(&'a str),
}

#[derive(RustEmbed)]
#[folder = "graphql"]
struct GraphQL;

fn load_graphql(file: &GQLFile<'_>) -> AniListResult<String> {
    let path = match file {
        GQLFile::Query(query) => format!("Queries/{}.graphql", query),
        GQLFile::Fragment(fragment) => format!("Fragments/{}.graphql", fragment),
    };
    let asset = GraphQL::get(&path).ok_or(Error::GraphQLLoadError)?;
    std::str::from_utf8(&asset)
        .map(str::to_string)
        .map_err(|_| Error::GraphQLLoadError)
}

fn query_from_parts(query_parts: Vec<GQLFile<'_>>) -> String {
    query_parts
        .iter()
        .map(load_graphql)
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
        .join("\n")
}

async fn make_request<V, T>(query_parts: Vec<GQLFile<'_>>, variables: V) -> AniListResult<T>
where
    V: Variables,
    T: DeserializeOwned,
{
    let query = query_from_parts(query_parts);
    let query_body = QueryBody {
        query,
        variables: Box::new(variables),
    };

    let http = ReqwestClient::new();
    let response = http.post(API_URL).json(&query_body).send().await?;

    Ok(response.json::<T>().await?)
}

macro_rules! check_anilist_errors {
    ($errors:expr) => {
        if let Some(errors) = $errors {
            return Err(Error::AniListErrors(errors));
        }
    };
}

pub async fn search_media(
    keyword: impl ToString,
    media_type: MediaType,
) -> AniListResult<Vec<Media>> {
    search_media_with_adult(keyword, media_type, false).await
}

pub async fn search_media_with_adult(
    keyword: impl ToString,
    media_type: MediaType,
    is_adult: bool,
) -> AniListResult<Vec<Media>> {
    use GQLFile::*;

    let query_parts = vec![
        Query("MediaSearch"),
        Fragment("MediaFull"),
        Fragment("MediaBase"),
        Fragment("PageInfo"),
    ];
    let variables = MediaVariables::default()
        .search(keyword)
        .is_adult(is_adult)
        .r#type(media_type)
        .to_owned();
    let response: PagedMediaResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    Ok(response.media())
}

pub async fn fetch_media(id: AniListID) -> AniListResult<Media> {
    fetch_media_with_adult(id, false).await
}

pub async fn fetch_media_with_adult(id: AniListID, is_adult: bool) -> AniListResult<Media> {
    use GQLFile::*;
    let query_parts = vec![
        Query("MediaFetch"),
        Fragment("MediaFull"),
        Fragment("MediaBase"),
        Fragment("PageInfo"),
    ];
    let variables = MediaVariables::default()
        .id(id)
        .is_adult(is_adult)
        .to_owned();
    let response: MediaResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    response.media().ok_or(Error::MediaNotFound(id))
}

pub async fn search_user(keyword: impl ToString) -> AniListResult<Vec<User>> {
    use GQLFile::*;

    let query_parts = vec![
        Query("UserSearch"),
        Fragment("MediaBase"),
        Fragment("CharacterBase"),
        Fragment("StaffBase"),
        Fragment("StudioBase"),
        Fragment("UserMediaStatistics"),
    ];
    let variables = StandardVariables::default().search(keyword).to_owned();
    let response: PagedUserResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    Ok(response.users())
}

pub async fn fetch_user(id: AniListID) -> AniListResult<User> {
    use GQLFile::*;

    let query_parts = vec![
        Query("UserFetch"),
        Fragment("MediaBase"),
        Fragment("CharacterBase"),
        Fragment("StaffBase"),
        Fragment("StudioBase"),
        Fragment("UserMediaStatistics"),
    ];
    let variables = StandardVariables::default().id(id).to_owned();
    let response: UserResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    response.user().ok_or(Error::UserNotFound(id))
}

pub async fn search_character(keyword: impl ToString) -> AniListResult<Vec<Character>> {
    use GQLFile::*;

    let query_parts = vec![
        Query("CharacterSearch"),
        Fragment("CharacterBase"),
        Fragment("MediaBase"),
    ];
    let variables = StandardVariables::default().search(keyword).to_owned();
    let response: PagedCharacterResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    Ok(response.characters())
}

pub async fn fetch_character(id: AniListID) -> AniListResult<Character> {
    use GQLFile::*;

    let query_parts = vec![
        Query("CharacterFetch"),
        Fragment("CharacterBase"),
        Fragment("MediaBase"),
    ];
    let variables = StandardVariables::default().id(id).to_owned();
    let response: CharacterResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    response.character().ok_or(Error::CharacterNotFound(id))
}

pub async fn search_staff(keyword: impl ToString) -> AniListResult<Vec<Staff>> {
    use GQLFile::*;

    let query_parts = vec![
        Query("StaffSearch"),
        Fragment("MediaBase"),
        Fragment("CharacterBase"),
    ];
    let variables = StandardVariables::default().search(keyword).to_owned();
    let response: PagedStaffResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    Ok(response.staff())
}

pub async fn fetch_staff(id: AniListID) -> AniListResult<Staff> {
    use GQLFile::*;

    let query_parts = vec![
        Query("StaffFetch"),
        Fragment("MediaBase"),
        Fragment("CharacterBase"),
    ];
    let variables = StandardVariables::default().id(id).to_owned();
    let response: StaffResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    response.staff().ok_or(Error::StaffNotFound(id))
}

pub async fn fetch_activity(id: AniListID) -> AniListResult<Activity> {
    use GQLFile::*;

    let query_parts = vec![
        Query("ActivityFetch"),
        Fragment("UserBase"),
        Fragment("MediaBase"),
    ];

    let variables = StandardVariables::default().id(id).to_owned();
    let response: ActivityResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    response.activity().ok_or(Error::ActivityNotFound(id))
}

pub async fn search_studio(keyword: impl ToString) -> AniListResult<Vec<Studio>> {
    use GQLFile::*;

    let query_parts = vec![Query("StudioSearch"), Fragment("MediaBase")];

    let variables = StandardVariables::default().search(keyword).to_owned();
    let response: PagedStudioResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    Ok(response.studios())
}

pub async fn fetch_studio(id: AniListID) -> AniListResult<Studio> {
    use GQLFile::*;

    let query_parts = vec![Query("StudioFetch"), Fragment("MediaBase")];

    let variables = StandardVariables::default().id(id).to_owned();
    let response: StudioResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    response.studio().ok_or(Error::StudioNotFound(id))
}

pub async fn fetch_airing_schedule_list(
    start_date: u64,
    end_date: u64,
) -> AniListResult<Vec<AiringSchedule>> {
    use GQLFile::*;
    let query_parts = vec![Query("AiringScheduleList"), Fragment("MediaFull")];
    let variables = AiringScheduleVariables::new(start_date, end_date);
    let response: PagedAiringScheduleResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    Ok(response.airing_schedules())
}

pub async fn fetch_airing_schedule_with_media_id(id: AniListID) -> AniListResult<AiringSchedule> {
    use GQLFile::*;
    let query_parts = vec![Query("AiringSchedule"), Fragment("MediaFull")];
    let variables = StandardVariables::default().id(id).to_owned();
    let response: AiringScheduleResponse = make_request(query_parts, variables).await?;

    check_anilist_errors!(response.errors);

    response
        .airing_schedule()
        .ok_or(Error::AiringScheduleNotFound(id))
}
