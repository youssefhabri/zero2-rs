mod blacklist;
mod dm;
mod echo;
mod embed;
mod log;
mod shutdown;

use serenity::framework::standard::macros::group;

use self::dm::*;
use self::echo::*;
use self::embed::*;
use self::log::*;
use self::shutdown::*;

group!({
    name: "System",
    commands: [dm, echo, embed, log, shutdown]
});
