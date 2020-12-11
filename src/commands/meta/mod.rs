mod avatar;

use serenity::framework::standard::macros::group;

use avatar::AVATAR_COMMAND;

#[group]
#[commands(avatar)]
struct Meta;
