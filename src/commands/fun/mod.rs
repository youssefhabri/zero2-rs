mod bigtext;
mod fortune_cookie;
mod next;
mod owoify;

use serenity::framework::standard::macros::group;

use bigtext::BIGTEXT_COMMAND;
use fortune_cookie::COOKIE_COMMAND;
use fortune_cookie::FORTUNE_COMMAND;
use next::NEXT_COMMAND;
use owoify::OWOIFY_COMMAND;

#[group]
#[commands(bigtext, cookie, fortune, next, owoify)]
struct Fun;
