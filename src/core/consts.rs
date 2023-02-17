#[cfg(feature = "db")]
use database::Database;
use once_cell::sync::Lazy;

pub static PREFIX: Lazy<String> =
    Lazy::new(|| kankyo::key("BOT_PREFIX").expect("bot PREFIX not found"));
pub static PREFIXES: Lazy<Vec<String>> = Lazy::new(|| load_csv_var("BOT_PREFIXES"));

#[cfg(feature = "db")]
lazy_static! {
    pub static ref DB: Database = Database::connect();
}

pub const OWNER_ID: u64 = 139360031102599168;
pub const BOT_ID: u64 = 453773001805135883;
pub const DEV_BOT_ID: u64 = 510136293968183317;

pub const MAIN_COLOUR: u32 = 16580705;

fn load_csv_var(name: &str) -> Vec<String> {
    match kankyo::key(name) {
        Some(value) => value.split(',').map(|v| v.to_string()).collect(),
        None => Vec::new(),
    }
}
