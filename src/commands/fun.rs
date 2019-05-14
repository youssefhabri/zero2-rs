use serenity::framework::standard::macros::group;

mod bigtext;
mod golendar;
mod fortune;

use self::bigtext::BIGTEXT_COMMAND;
use self::golendar::GOLENDAR_COMMAND;
use self::fortune::FORTUNE_COMMAND;

group!({
    name: "Fun",
    commands: [bigtext, golendar, fortune]
});
