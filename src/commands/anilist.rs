use serenity::framework::standard::macros::group;


pub mod utils;
pub mod client;

// command modules
pub mod activity;
pub mod airing;
pub mod anime;
pub mod character;
pub mod manga;
pub mod user;

use self::activity::ACTIVITY_COMMAND;
use self::airing::AIRING_COMMAND;
use self::anime::ANIME_COMMAND;
use self::character::CHARACTER_COMMAND;
use self::manga::MANGA_COMMAND;
use self::user::USER_COMMAND;

group!({
    name: "Anilist",
    commands: [activity, airing, anime, character, manga, user]
});