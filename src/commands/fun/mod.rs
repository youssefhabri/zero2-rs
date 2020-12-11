mod bigtext;
mod fortune_cookie;
mod giphy;
mod next;
mod nlimage;
mod owoify;

use serenity::framework::standard::macros::group;

use bigtext::BIGTEXT_COMMAND;
use fortune_cookie::COOKIE_COMMAND;
use fortune_cookie::FORTUNE_COMMAND;
use giphy::GIPHY_COMMAND;
use next::NEXT_COMMAND;
use nlimage::NLIMAGE_COMMAND;
use owoify::OWOIFY_COMMAND;

#[group]
#[commands(bigtext, cookie, fortune, giphy, next, nlimage, owoify)]
struct Fun;
