use serenity::framework::standard::macros::group;

mod urban;

use urban::URBAN_COMMAND;

#[group]
#[commands(urban)]
struct Knowledge;
