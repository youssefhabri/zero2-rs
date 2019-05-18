use serenity::framework::standard::macros::group;

mod bigtext;
mod fortune;
mod golendar;

use self::bigtext::BIGTEXT_COMMAND;
use self::fortune::FORTUNE_COMMAND;
use self::golendar::GOLENDAR_COMMAND;

group!({
    name: "Fun",
    commands: [bigtext, golendar, fortune]
});
