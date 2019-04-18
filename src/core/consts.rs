lazy_static! {
    pub static ref PREFIX: String = dotenv::var("BOT_PREFIX").expect("token");
}

/// AniTrend Bots ids for reference
pub const BOT_IDS: [u64; 4] = [453773001805135883, 510136293968183317, 510000124949168165, 235088799074484224];
