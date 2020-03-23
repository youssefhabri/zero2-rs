use std::collections::HashMap;

use crate::db::Database;
use crate::models::emoji::Emoji;

lazy_static! {
    pub static ref DB: Database = Database::connect();
    pub static ref COOKIES: Vec<String> = load_cookies();
    pub static ref EMOJIS: HashMap<String, Emoji> = load_emojis();
    pub static ref OWNER_ID: String = dotenv::var("OWNER_ID").expect("OWNER_ID");
    pub static ref PREFIX: String = dotenv::var("BOT_PREFIX").expect("BOT_PREFIX");
    pub static ref PREFIXES: Vec<String> = load_prefixes();
}

pub const BOT_ID: u64 = 453773001805135883;

/// AniTrend Bots ids for reference
pub const AT_BOT_IDS: [u64; 4] = [
    453773001805135883,
    510136293968183317,
    510000124949168165,
    235088799074484224,
];

pub const MAIN_COLOUR: u32 = 16580705;

fn load_prefixes() -> Vec<String> {
    match dotenv::var("BOT_PREFIXES") {
        Ok(prefixes_str) => prefixes_str
            .split(',')
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
        Err(_) => vec![],
    }
}

fn load_cookies() -> Vec<String> {
    let url = "https://raw.githubusercontent.com/ianli/fortune-cookies-galore/master/fortunes.txt";
    let mut cookies = vec![];
    match reqwest::blocking::get(url) {
        Ok(res) => match res.text() {
            Ok(text) => {
                text.split('\n').for_each(|s| {
                    cookies.push(s.to_string());
                });
            }
            Err(why) => error!("Error extracting cookies: {:?}", why),
        },
        Err(why) => error!("Error fetching cookies file: {:?}", why),
    }
    cookies
}

fn load_emojis() -> HashMap<String, Emoji> {
    let url = "https://discordemoji.com/api/";
    let mut emojis: Vec<Emoji> = vec![];
    match reqwest::blocking::get(url) {
        Ok(response) => match response.json::<Vec<Emoji>>() {
            Ok(data) => emojis = data,
            Err(why) => error!("Error parsing emojis from DiscordEmoji: {:?}", why),
        },
        Err(why) => error!("Error fetching emojis from DiscordEmoji: {:?}", why),
    }

    emojis
        .into_iter()
        .map(|emoji| (emoji.title.clone(), emoji))
        .collect()
}
