use serenity::framework::standard::macros::group;

mod bigtext;
mod cookie;
mod fortune;
mod golendar;
mod next;
mod owo;

use self::bigtext::BIGTEXT_COMMAND;
use self::cookie::COOKIE_COMMAND;
use self::fortune::FORTUNE_COMMAND;
use self::golendar::GOLENDAR_COMMAND;
use self::next::NEXT_COMMAND;
use self::owo::OWO_COMMAND;

#[group]
#[commands(bigtext, cookie, fortune, golendar, next, owo)]
struct Fun;
