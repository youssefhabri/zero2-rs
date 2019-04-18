use serenity::framework::standard::CreateGroup;

pub mod anilist;
pub mod giphy;
pub mod fun;
pub mod meta;
pub mod nekoslife;
pub mod urban;


pub fn init_no_category() -> CreateGroup {
    CreateGroup::default()
        .command("gif", |c| c.cmd(giphy::GiphyCommand))
}