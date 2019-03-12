use std::collections::HashMap;
use std::str;

use crate::models::anilist::{
    Variables,
    QueryBody,
    Response,
    activity::Activity,
    airing_schedule::AiringSchedule,
    character::Character,
    media::Media,
    user::User,
};


#[derive(RustEmbed)]
#[folder = "assets/graphql"]
struct GraphQL;

fn load_graphql(file: &str) -> String {
    let asset = match GraphQL::get(file) {
        Some(asset) => asset,
        None => panic!("Error loading query: {}", file)
    };
    str::from_utf8(&asset).expect(file).to_owned()
}

fn load_graphql_with_fragment(query_file: &str, fragment_files: Vec<&str>) -> String {
    let query = load_graphql(&format!("queries/{}.graphql", query_file));

    let mut fragments: Vec<String> = vec![];
    for fragment in fragment_files {
        fragments.push(
            load_graphql(&format!("fragments/{}.graphql", fragment))
        );
    }

    format!("{}\n{}",
            query,
            fragments.join("\n"))
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
    let media_query = load_graphql("queries/Search/MediaSearch.graphql");
    let mut variables = HashMap::new();
    variables.insert("search".to_owned(), keyword);
    variables.insert("type".to_owned(), media_type);

    query(media_query, variables).data.page.media()
}

pub fn search_media_by_id(media_id: String, media_type: String) -> Option<Media> {
    let media_query = load_graphql("queries/MediaQuery.graphql");
    let mut variables = HashMap::new();
    variables.insert("id".to_owned(), media_id);
    variables.insert("type".to_owned(), media_type);

    query(media_query, variables).data.media
}

pub fn search_users(keyword: String) -> Vec<User> {
    let user_query = load_graphql_with_fragment("Search/UserSearch", vec!["MediaBase"]);
    let mut variables = HashMap::new();
    variables.insert("search".to_owned(), keyword);

    query(user_query, variables).data.page.users()
}

pub fn search_user(username: String) -> Option<User> {
    let user_query = load_graphql_with_fragment("UserQuery", vec!["MediaBase"]);
    let mut variables = HashMap::new();
    variables.insert("username".to_owned(), username);

    query(user_query, variables).data.user
}

pub fn search_characters(keyword: String) -> Vec<Character> {
    let character_query = load_graphql_with_fragment("Search/CharacterSearch", vec!["MediaBase"]);
    let mut variables = HashMap::new();
    variables.insert("search".to_owned(), keyword);

    query(character_query, variables).data.page.characters()
}

pub fn search_character_by_id(character_id: String) -> Option<Character> {
    let character_query = load_graphql_with_fragment("CharacterQuery", vec!["MediaBase"]);
    let mut variables = HashMap::new();
    variables.insert("id".to_owned(), character_id);

    query(character_query, variables).data.character
}

pub fn search_activity(activity_id: String) -> Option<Activity> {
    let activity_query = load_graphql_with_fragment("ActivityQuery", vec!["MediaBase", "UserBase"]);
    let mut variables = HashMap::new();
    variables.insert("id".to_owned(), activity_id);

    query(activity_query, variables).data.activity
}

pub fn search_airing_schedule(start_time: i64, end_time: i64) -> Vec<AiringSchedule> {
    let activity_query = load_graphql_with_fragment("AiringSchedule", vec!["MediaBase"]);
    let mut variables = HashMap::new();
    variables.insert("start".to_owned(), format!("{}", start_time));
    variables.insert("end".to_owned(), format!("{}", end_time));

    query(activity_query, variables).data.page.airing_schedule()
}
