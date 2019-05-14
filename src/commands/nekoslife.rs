use serenity::framework::standard::macros::group;

pub mod nlowo;
pub mod nlimage;

use self::nlowo::NLOWO_COMMAND;
use self::nlimage::NLIMAGE_COMMAND;

group!({
    name: "NekosLife",
    commands: [nlowo, nlimage]
});