use serenity::framework::standard::macros::group;

pub mod nlimage;
pub mod nlowo;

use self::nlimage::NLIMAGE_COMMAND;
use self::nlowo::NLOWO_COMMAND;

group!({
    name: "NekosLife",
    commands: [nlowo, nlimage]
});
