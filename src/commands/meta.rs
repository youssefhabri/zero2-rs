use serenity::framework::standard::macros::group;

mod avatar;
mod botinfo;
mod ping;
mod stats;
mod test;

use self::avatar::AVATAR_COMMAND;
use self::botinfo::BOTINFO_COMMAND;
use self::ping::PING_COMMAND;
use self::stats::STATS_COMMAND;
use self::test::TEST_COMMAND;

group!({
    name: "Meta",
    commands: [avatar, botinfo, ping, stats, test]
});
