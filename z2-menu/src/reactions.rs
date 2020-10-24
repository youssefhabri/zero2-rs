use serenity::model::prelude::ReactionType;

pub const PREV: &str = "⬅";
pub const NEXT: &str = "➡";
pub const FIRST: &str = "⏮️";
pub const LAST: &str = "⏭️";
pub const STOP: &str = "❌";

pub const ANIME: &str = "🇦";
pub const MANGA: &str = "🇲";

pub const OVERVIEW: &str = "🇴";
// pub const CHARACTERS: &str = "🇨";
pub const FAVOURITES: &str = "🇫";
pub const STATS: &str = "🇸";
pub const RECOMMENDATIONS: &str = "🇷";

fn _single_page_reactions(mut reactions: Vec<&str>) -> Vec<ReactionType> {
    reactions.push(STOP);

    reactions
        .iter()
        .map(|r| ReactionType::Unicode(r.to_string()))
        .collect()
}

fn _few_pages_reactions(reactions: Vec<&str>) -> Vec<ReactionType> {
    let mut new_reactions = vec![PREV];
    new_reactions.append(&mut reactions.to_vec());
    new_reactions.append(&mut vec![NEXT, STOP]);

    new_reactions
        .iter()
        .map(|r| ReactionType::Unicode(r.to_string()))
        .collect()
}

fn _many_pages_reactions(reactions: Vec<&str>) -> Vec<ReactionType> {
    let mut new_reactions = vec![FIRST, PREV];
    new_reactions.append(&mut reactions.to_vec());
    new_reactions.append(&mut vec![NEXT, LAST, STOP]);

    new_reactions
        .iter()
        .map(|r| ReactionType::Unicode(r.to_string()))
        .collect()
}

pub fn _default<'a>() -> Vec<ReactionType> {
    [FIRST, PREV, NEXT, LAST, STOP]
        .iter()
        .map(|r| ReactionType::Unicode(r.to_string()))
        .collect()
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
