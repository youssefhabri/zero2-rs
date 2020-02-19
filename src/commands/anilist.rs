use serenity::framework::standard::macros::group;

pub mod client;
pub mod utils;

// command modules
pub mod activity;
pub mod airing;
pub mod anime;
pub mod character;
pub mod manga;
pub mod staff;
pub mod user;

use self::activity::ACTIVITY_COMMAND;
use self::airing::AIRING_COMMAND;
use self::anime::ANIME_COMMAND;
use self::character::CHARACTER_COMMAND;
use self::manga::MANGA_COMMAND;
use self::staff::STAFF_COMMAND;
use self::user::USER_COMMAND;

#[group]
#[commands(activity, airing, anime, character, manga, staff, user)]
struct Anilist;
