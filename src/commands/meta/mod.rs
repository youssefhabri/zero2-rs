mod avatar;
mod bot_info;
mod ping;
mod who;

use serenity::framework::standard::macros::group;

use avatar::AVATAR_COMMAND;
use bot_info::BOT_INFO_COMMAND;
use ping::PING_COMMAND;
use who::WHO_COMMAND;

#[group]
#[commands(avatar, bot_info, ping, who)]
struct Meta;
