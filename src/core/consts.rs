use crate::db::Database;

lazy_static! {
    pub static ref PREFIX: String = dotenv::var("BOT_PREFIX").expect("BOT_PREFIX");
    pub static ref PREFIXES: Vec<String> = {
        match dotenv::var("BOT_PREFIXES") {
            Ok(prefixes_str) => prefixes_str
                .split(',')
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            Err(_) => vec![],
        }
    };
    pub static ref DB: Database = Database::connect();
    pub static ref OWNER_ID: String = dotenv::var("OWNER_ID").expect("OWNER_ID");
    pub static ref COOKIES: Vec<String> = {
        let url =
            "https://raw.githubusercontent.com/ianli/fortune-cookies-galore/master/fortunes.txt";
        let mut cookies = vec![];
        match reqwest::get(url) {
            Ok(mut res) => match res.text() {
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
    };
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
