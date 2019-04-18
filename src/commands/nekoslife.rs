use serenity::framework::standard::CreateGroup;

pub mod nlowo;
pub mod nlimage;


pub fn init_nekoslife() -> CreateGroup {
    CreateGroup::default()
        .command("nekoslife", |c| c
            .cmd(nlimage::NLImageCommand)
            .batch_known_as(vec!["nl", "nlimg"])
            .desc("Get gifs from nekos.life.")
            .usage("nl [keyword:optional] [user:optional]")
        )
        .command("owo", |c| c
            .cmd(nlowo::NLOwOCommand)
            .batch_known_as(vec!["nlowo"])
            .desc("Get gifs from nekos.life.")
            .usage("owo [text]")
        )
}

