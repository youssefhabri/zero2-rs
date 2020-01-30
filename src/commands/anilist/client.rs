use std::str;

use crate::models::anilist::{
    activity::Activity,
    airing_schedule::AiringSchedule,
    character::Character,
    media::{Media, MediaType},
    staff::Staff,
    studio::Studio,
    user::User,
    QueryBody, Response,
};

#[derive(RustEmbed)]
#[folder = "assets/graphql"]
struct GraphQL;

fn load_graphql(file: &str) -> String {
    let asset = match GraphQL::get(file) {
        Some(asset) => asset,
        None => panic!("Error loading query: {}", file),
    };
    str::from_utf8(&asset).expect(file).to_owned()
}

fn load_graphql_with_fragment(query_file: &str, fragment_files: Vec<&str>) -> String {
    let query = load_graphql(&format!("queries/{}.graphql", query_file));

    let mut fragments: Vec<String> = vec![];
    for fragment in fragment_files {
        fragments.push(load_graphql(&format!("fragments/{}.graphql", fragment)));
    }

    format!("{}\n{}", query, fragments.join("\n"))
}

type Var<'a> = (&'a str, &'a str);

pub fn query(query: &str, variables: Vec<Var>, fragments: Vec<&str>) -> Response {
    let body = QueryBody {
        query: load_graphql_with_fragment(query, fragments),
        variables: variables
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    };

    let client = reqwest::blocking::Client::new();
    let mut res = client
        .post("https://graphql.anilist.co")
        .json(&body)
        .send()
        .expect("response");

    res.json::<Response>().expect("json")
}

pub fn search_media(keyword: String, media_type: MediaType) -> Vec<Media> {
    query(
        "Search/MediaSearch",
        vec![
            ("search", keyword.as_str()),
            ("type", &media_type.to_string()),
        ],
        vec!["MediaBase"],
    )
    .data
    .page
    .media()
}

pub fn search_media_by_id(media_id: String, media_type: String) -> Option<Media> {
    query(
        "MediaQuery",
        vec![("id", media_id.as_str()), ("type", media_type.as_str())],
        vec!["MediaBase"],
    )
    .data
    .media
}

pub fn search_users(keyword: String) -> Vec<User> {
    query(
        "Search/UserSearch",
        vec![("search", keyword.as_str())],
        vec!["CharacterBase", "MediaBase", "UserStatistics"],
    )
    .data
    .page
    .users()
}

pub fn search_user(username: String) -> Option<User> {
    query(
        "UserQuery",
        vec![("username", username.as_str())],
        vec!["CharacterBase", "MediaBase", "UserStatistics"],
    )
    .data
    .user
}

pub fn search_characters(keyword: String) -> Vec<Character> {
    query(
        "Search/CharacterSearch",
        vec![("search", keyword.as_str())],
        vec!["CharacterBase", "MediaBase"],
    )
    .data
    .page
    .characters()
}

pub fn search_character_by_id(character_id: String) -> Option<Character> {
    query(
        "CharacterQuery",
        vec![("id", character_id.as_str())],
        vec!["CharacterBase", "MediaBase"],
    )
    .data
    .character
}

pub fn search_activity(activity_id: String) -> Option<Activity> {
    query(
        "ActivityQuery",
        vec![("id", activity_id.as_str())],
        vec!["MediaBase", "UserBase"],
    )
    .data
    .activity
}

pub fn search_airing_schedule(start_time: i64, end_time: i64) -> Vec<AiringSchedule> {
    query(
        "AiringSchedule",
        vec![
            ("start", start_time.to_string().as_str()),
            ("end", end_time.to_string().as_str()),
        ],
        vec!["MediaBase"],
    )
    .data
    .page
    .airing_schedule()
}

pub fn search_studio(id: String) -> Option<Studio> {
    query("StudioQuery", vec![("id", id.as_str())], vec!["MediaBase"])
        .data
        .studio
}

pub fn search_staff(keyword: String) -> Vec<Staff> {
    query(
        "Search/StaffSearch",
        vec![("search", keyword.as_str())],
        vec!["CharacterBase", "MediaBase"],
    )
    .data
    .page
    .staff()
}

pub fn search_staff_by_id(staff_id: String) -> Option<Staff> {
    query(
        "StaffQuery",
        vec![("id", staff_id.as_str())],
        vec!["CharacterBase", "MediaBase"],
    )
    .data
    .staff
}
