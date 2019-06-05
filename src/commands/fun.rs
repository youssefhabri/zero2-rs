use serenity::framework::standard::macros::group;

mod bigtext;
mod fortune;
mod golendar;
mod owo;

use self::bigtext::BIGTEXT_COMMAND;
use self::fortune::FORTUNE_COMMAND;
use self::golendar::GOLENDAR_COMMAND;
use self::owo::OWO_COMMAND;

group!({
    name: "Fun",
    commands: [bigtext, golendar, fortune, owo]
});
