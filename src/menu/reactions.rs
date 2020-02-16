pub const PREV: &str = "⬅";
pub const NEXT: &str = "➡";
pub const FIRST: &str = "⏮️";
pub const LAST: &str = "⏭️";
pub const STOP: &str = "❌";

pub const ANIME: &str = "🇦";
pub const MANGA: &str = "🇲";

pub const OVERVIEW: &str = "🇴";
pub const CHARACTERS: &str = "";

pub fn default<'a>() -> Vec<&'a str> {
    [FIRST, PREV, NEXT, LAST, STOP].to_vec()
}
pub fn stats<'a>() -> Vec<&'a str> {
    [ANIME, MANGA, STOP].to_vec()
}
pub fn media<'a>() -> Vec<&'a str> {
    [OVERVIEW, CHARACTERS, STOP].to_vec()
}
