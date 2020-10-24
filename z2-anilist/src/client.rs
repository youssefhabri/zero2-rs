use reqwest::{Client as ReqwestClient, Error as ReqwestError};
use rust_embed::RustEmbed;
use serde::{de::DeserializeOwned, Serialize};
use std::result::Result as StdResult;
use thiserror::Error;

use crate::models::shared::{
    AniListError, AniListID, CharacterResponse, MediaResponse, PagedCharacterResponse,
    PagedMediaResponse, PagedStaffResponse, PagedUserResponse, StaffResponse, UserResponse,
};
use crate::models::{Character, Media, MediaType, Staff, User};
use crate::query_variables::{MediaVariables, StandardVariables, Variables};

const API_URL: &str = "https://graphql.anilist.co";

#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] ReqwestError),

    #[error("AniList Media (id: {0}) not found")]
    MediaNotFound(AniListID),

    #[error("AniList User (id: {0}) not found")]
    UserNotFound(AniListID),

    #[error("AniList Character (id: {0}) not found")]
    CharacterNotFound(AniListID),

    #[error("AniList Staff (id: {0}) not found")]
    StaffNotFound(AniListID),

    #[error("AniList returned some errors: {0:?}")]
    AniListErrors(Vec<AniListError>),

    #[error("Error loading the graphql file")]
    GraphQLLoadError,
}

type AniListResult<T> = StdResult<T, Error>;

#[derive(Serialize)]
pub struct QueryBody {
    pub query: String,
    pub variables: Box<dyn Variables>,
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
        .map(|asset| asset.to_string())
        .map_err(|_| Error::GraphQLLoadError)
}

fn query_from_parts(query_parts: Vec<GQLFile<'_>>) -> AniListResult<String> {
    let query = query_parts
        .iter()
        .filter_map(|file| load_graphql(file).ok())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(query)
}

async fn make_request<T: DeserializeOwned>(
    query_parts: Vec<GQLFile<'_>>,
    variables: Box<dyn Variables>,
) -> AniListResult<T> {
    let query = query_from_parts(query_parts)?;
    let query_body = QueryBody { query, variables };

    let http = ReqwestClient::new();
    let response = http.post(API_URL).json(&query_body).send().await?;

    Ok(response.json::<T>().await?)
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
        .clone();
    let response: PagedMediaResponse = make_request(query_parts, Box::new(variables)).await?;

    if let Some(errors) = response.errors {
        return Err(Error::AniListErrors(errors));
    }

    Ok(response.media().clone())
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
    let variables = MediaVariables::default().id(id).is_adult(is_adult).clone();
    let response: MediaResponse = make_request(query_parts, Box::new(variables)).await?;

    response.media().clone().ok_or(Error::MediaNotFound(id))
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
    let variables = StandardVariables::default().search(keyword).clone();
    let response: PagedUserResponse = make_request(query_parts, Box::new(variables)).await?;

    if let Some(errors) = response.errors {
        return Err(Error::AniListErrors(errors));
    }

    Ok(response.users().clone())
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
    let variables = StandardVariables::default().id(id).clone();
    let response: UserResponse = make_request(query_parts, Box::new(variables)).await?;

    response.user().clone().ok_or(Error::UserNotFound(id))
}

pub async fn search_character(keyword: impl ToString) -> AniListResult<Vec<Character>> {
    use GQLFile::*;

    let query_parts = vec![
        Query("CharacterFetch"),
        Fragment("CharacterBase"),
        Fragment("MediaBase"),
    ];
    let variables = StandardVariables::default().search(keyword).clone();
    let response: PagedCharacterResponse = make_request(query_parts, Box::new(variables)).await?;

    if let Some(errors) = response.errors {
        return Err(Error::AniListErrors(errors));
    }

    Ok(response.characters().clone())
}

pub async fn fetch_character(id: AniListID) -> AniListResult<Character> {
    use GQLFile::*;

    let query_parts = vec![
        Query("CharacterFetch"),
        Fragment("CharacterBase"),
        Fragment("MediaBase"),
    ];
    let variables = StandardVariables::default().id(id).clone();
    let response: CharacterResponse = make_request(query_parts, Box::new(variables)).await?;

    response
        .character()
        .clone()
        .ok_or(Error::CharacterNotFound(id))
}

pub async fn search_staff(keyword: impl ToString) -> AniListResult<Vec<Staff>> {
    use GQLFile::*;

    let query_parts = vec![
        Query("StaffSearch"),
        Fragment("MediaBase"),
        Fragment("CharacterBase"),
    ];
    let variables = StandardVariables::default().search(keyword).clone();
    let response: PagedStaffResponse = make_request(query_parts, Box::new(variables)).await?;

    if let Some(errors) = response.errors {
        return Err(Error::AniListErrors(errors));
    }

    Ok(response.staff().clone())
}

pub async fn fetch_staff(id: AniListID) -> AniListResult<Staff> {
    use GQLFile::*;

    let query_parts = vec![
        Query("StaffFetch"),
        Fragment("MediaBase"),
        Fragment("CharacterBase"),
    ];
    let variables = StandardVariables::default().id(id).clone();
    let response: StaffResponse = make_request(query_parts, Box::new(variables)).await?;

    response.staff().clone().ok_or(Error::StaffNotFound(id))
}
