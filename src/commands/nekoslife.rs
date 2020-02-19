use serenity::framework::standard::macros::group;

pub mod nlimage;

use self::nlimage::NLIMAGE_COMMAND;

#[group]
#[commands(nlimage)]
struct NekosLife;
