use database::Database;

lazy_static! {
    pub static ref DB: Database = Database::connect();
    pub static ref PREFIX: String = kankyo::key("BOT_PREFIX").expect("bot PREFIX not found");
    pub static ref PREFIXES: Vec<String> = load_csv_var("BOT_PREFIXES");
}

pub const OWNER_ID: u64 = 139360031102599168;
pub const BOT_ID: u64 = 453773001805135883;

pub const MAIN_COLOUR: u32 = 16580705;

fn load_csv_var(name: &str) -> Vec<String> {
    match kankyo::key(name) {
        Some(value) => value.split(',').map(|v| v.to_string()).collect(),
        None => Vec::new(),
    }
}
