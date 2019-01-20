use std::collections::HashMap;
use std::str;

use crate::commands::anilist::models::{
    Variables,
    QueryBody,
    Response,
    media::Media
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
    // TODO fix graphql path
    //  * add a helper function to handle loading graphql files
    let media_query = load_graphql("MediaSearch");
    let mut variables = HashMap::new();
    variables.insert("search".to_owned(), keyword);
    variables.insert("type".to_owned(), media_type);

    query(media_query, variables).data.page.media()
}