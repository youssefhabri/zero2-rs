use serenity::framework::standard::CreateGroup;

pub mod utils;
pub mod client;

// command modules
pub mod activity;
pub mod airing;
pub mod anime;
pub mod character;
pub mod manga;
pub mod user;


pub fn init_anilist() -> CreateGroup {
    CreateGroup::default()
        .command("anime", |c| c
            .cmd(anime::AnimeCommand)
            .batch_known_as(vec!["a"])
            .usage("<anime>")
            .desc("Search for an anime in AniList")
        )
        .command("manga", |c| c
            .cmd(manga::MangaCommand)
            .batch_known_as(vec!["m"])
            .usage("<manga>")
            .desc("Search for a manga in AniList")
        )
        .command("user", |c| c
            .cmd(user::UserCommand)
            .batch_known_as(vec!["u"])
            .usage("<user>")
            .desc("Search for a user in AniList")
        )
        .command("character", |c| c
            .cmd(character::CharacterCommand)
            .batch_known_as(vec!["c"])
            .usage("<character>")
            .desc("Search for a character in AniList")
        )
        .command("activity", |c| c
            .cmd(activity::ActivityCommand)
            .batch_known_as(vec!["act"])
            .usage("<activity_id|activity_url>")
            .desc("Embed an activity from AniList")
        )
        .command("airing", |c| c
            .cmd(airing::AiringCommand)
            .batch_known_as(vec!["airs"])
            .usage("[weekday]")
            .desc("Embed an activity from AniList")
        )
}