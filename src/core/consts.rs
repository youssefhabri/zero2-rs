use crate::db::Database;

lazy_static! {
    pub static ref PREFIX: String = dotenv::var("BOT_PREFIX").expect("token");
    pub static ref DB: Database = Database::connect();
    pub static ref COOKIES: Vec<String> = {
        let url =
            "https://raw.githubusercontent.com/ianli/fortune-cookies-galore/master/fortunes.txt";
        let mut cookies = vec![];
        match reqwest::get(url) {
            Ok(mut res) => match res.text() {
                Ok(text) => {
                    text.split("\n").for_each(|s| {
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
