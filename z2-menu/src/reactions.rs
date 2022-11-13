use chrono::Weekday;
use serenity::model::prelude::ReactionType;

pub const PREV: &str = "⬅";
pub const NEXT: &str = "➡";
pub const FIRST: &str = "⏮️";
pub const LAST: &str = "⏭️";
pub const STOP: &str = "❌";
pub const HOME: &str = "🔢";

pub const DELETE: &str = "🗑️";

pub const ONE: &str = "1⃣";
pub const TWO: &str = "2⃣";
pub const THREE: &str = "3⃣";
pub const FOUR: &str = "4⃣";
pub const FIVE: &str = "5⃣";
pub const SIX: &str = "6⃣";
pub const SEVEN: &str = "7⃣";

pub const ANIME: &str = "🇦";
pub const MANGA: &str = "🇲";

pub const OVERVIEW: &str = "🇴";
// pub const CHARACTERS: &str = "🇨";
pub const FAVOURITES: &str = "🇫";
pub const STATS: &str = "🇸";
pub const RECOMMENDATIONS: &str = "🇷";

fn _single_page_reactions(mut reactions: Vec<&str>) -> Vec<ReactionType> {
    reactions.append(&mut vec![STOP, DELETE]);

    reactions
        .into_iter()
        .map(str::to_string)
        .map(ReactionType::Unicode)
        .collect()
}

fn _few_pages_reactions(reactions: Vec<&str>) -> Vec<ReactionType> {
    let mut new_reactions = vec![PREV];
    new_reactions.append(&mut reactions.to_vec());
    new_reactions.append(&mut vec![NEXT, STOP, DELETE]);

    new_reactions
        .into_iter()
        .map(str::to_string)
        .map(ReactionType::Unicode)
        .collect()
}

fn _many_pages_reactions(reactions: Vec<&str>) -> Vec<ReactionType> {
    let mut new_reactions = vec![FIRST, PREV];
    new_reactions.append(&mut reactions.to_vec());
    new_reactions.append(&mut vec![NEXT, LAST, STOP, DELETE]);

    new_reactions
        .into_iter()
        .map(str::to_string)
        .map(ReactionType::Unicode)
        .collect()
}

pub fn default(num_pages: usize) -> Vec<ReactionType> {
    make_reactions(vec![], num_pages)
}

pub fn make_reactions(reactions: Vec<&str>, number_of_pages: usize) -> Vec<ReactionType> {
    match number_of_pages {
        1 => _single_page_reactions(reactions),
        2..=10 => _few_pages_reactions(reactions),
        _ => _many_pages_reactions(reactions),
    }
}

pub fn media(num_pages: usize) -> Vec<ReactionType> {
    make_reactions(vec![OVERVIEW, STATS, RECOMMENDATIONS], num_pages)
}

pub fn user(num_pages: usize) -> Vec<ReactionType> {
    make_reactions(vec![OVERVIEW, STATS, FAVOURITES], num_pages)
}

pub fn character(num_pages: usize) -> Vec<ReactionType> {
    make_reactions(vec![OVERVIEW, ANIME, MANGA], num_pages)
}

pub fn staff(num_pages: usize) -> Vec<ReactionType> {
    make_reactions(vec![OVERVIEW, ANIME, MANGA], num_pages)
}

pub fn airing_schedule_main() -> Vec<ReactionType> {
    _single_page_reactions(vec![ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN])
}

pub fn airing_schedule_media(num_pages: usize) -> Vec<ReactionType> {
    make_reactions(vec![HOME], num_pages)
}

pub fn airing_schedule_from_weekday(
    weekday: Option<Weekday>,
    num_pages: usize,
) -> Vec<ReactionType> {
    match weekday {
        Some(_) => make_reactions(vec![HOME], num_pages),
        None => airing_schedule_main(),
    }
}
