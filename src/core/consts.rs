use regex::Regex;
use std::collections::HashMap;

use crate::db::Database;
use crate::models::emoji::Emoji;

lazy_static! {
    pub static ref DB: Database = Database::connect();
    pub static ref COOKIES: Vec<String> = load_cookies();
    pub static ref EMOJIS: HashMap<String, Emoji> = load_emojis();
    pub static ref OWNER_ID: String = dotenv::var("OWNER_ID").expect("OWNER_ID");
    pub static ref PREFIX: String = dotenv::var("BOT_PREFIX").expect("BOT_PREFIX");
    pub static ref PREFIXES: Vec<String> = load_csv_var("BOT_PREFIXES");
    pub static ref VIP_ROLES: Vec<String> = load_csv_var("VIP_ROLES");

    // RegEx
    // TODO move the rest of the regexes here
    pub static ref MESSAGE_ID_RE: Regex = Regex::new(r"[0-9]{17,18}").unwrap();
    pub static ref MESSAGE_LINK_RE: Regex = Regex::new(r"https://.*discordapp\.com/channels/([0-9]*)/([0-9]*)/([0-9]*)/?").unwrap();
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

fn load_csv_var(key: &str) -> Vec<String> {
    match dotenv::var(key) {
        Ok(prefixes_str) => prefixes_str
            .split(',')
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
        Err(_) => vec![],
    }
}

// TODO try and cache the file locally and only download it if it was changed/after a period of time
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

// TODO maybe remove this as there no longer a need for the bot to try and post emojis for non-nitro users
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
