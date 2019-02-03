use std::collections::HashMap;
use std::str;

use crate::commands::anilist::models::{
    Variables,
    QueryBody,
    Response,
    character::Character,
    media::Media,
    user::User,
    activity::Activity
};


#[derive(RustEmbed)]
#[folder = "assets/queries"]
struct Query;


fn load_graphql(file: &str) -> String {
    let asset = match Query::get(&format!("{}.graphql", file)) {
        Some(asset) => asset,
        None => panic!("Error loading: {}", file)
    };
    str::from_utf8(&asset).expect(file).to_owned()
}


pub fn query(query: String, variables: Variables) -> Response {
    let body = QueryBody {
        query, variables
    };

    let client = reqwest::Client::new();
    let mut res = client.post("https://graphql.anilist.co")
        .json(&body)
        .send().expect("response");
    let response: Response = res.json().expect("json");

    response
}

pub fn search_media(keyword: String, media_type: String) -> Vec<Media> {
    let media_query = load_graphql("MediaSearch");
    let mut variables = HashMap::new();
    variables.insert("search".to_owned(), keyword);
    variables.insert("type".to_owned(), media_type);

    query(media_query, variables).data.page.media()
}

pub fn search_user(keyword: String) -> Vec<User> {
    let user_query = load_graphql("UserSearch");
    let mut variables = HashMap::new();
    variables.insert("search".to_owned(), keyword);

    query(user_query, variables).data.page.users()
}

pub fn search_character(keyword: String) -> Vec<Character> {
    let character_query = load_graphql("CharacterSearch");
    let mut variables = HashMap::new();
    variables.insert("search".to_owned(), keyword);

    query(character_query, variables).data.page.characters()
}

pub fn search_activity(activity_id: String) -> Option<Activity> {
    let activity_query = load_graphql("ActivitySearch");
    let mut variables = HashMap::new();
    variables.insert("id".to_owned(), activity_id);

    query(activity_query, variables).data.activity
}
