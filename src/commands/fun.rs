use serenity::framework::standard::macros::group;

mod golendar;
mod fortune;

use self::golendar::GOLENDAR_COMMAND;
use self::fortune::FORTUNE_COMMAND;

group!({
    name: "Fun",
    commands: [golendar, fortune]
});
