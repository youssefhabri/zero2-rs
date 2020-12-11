mod avatar;
mod who;

use serenity::framework::standard::macros::group;

use avatar::AVATAR_COMMAND;
use who::WHO_COMMAND;

#[group]
#[commands(avatar, who)]
struct Meta;
