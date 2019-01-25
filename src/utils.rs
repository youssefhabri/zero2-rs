use serenity::builder::CreateMessage;
use serenity::prelude::*;

/*
 * TODO Create a menu system to handle search results
 *  - Will probably need to wait for `async` update in serenity
 *  -
 */


struct MenuControls {
    next: String,
    prev: String,
    stop: String,
}

struct Menu;

impl Menu {
    pub fn new(
        ctx: Context,
        pages: Vec<CreateMessage>,
        controls: MenuControls,
        message: Message,
        page: u8,
        // emoji: String // TODO do we need this?
    ) {}
}