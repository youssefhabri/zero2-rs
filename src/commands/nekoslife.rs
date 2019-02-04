use serenity::framework::StandardFramework;

pub mod nlowo;
pub mod nlimage;


pub fn register(framework: StandardFramework) -> StandardFramework {
    framework.group("Nekos.life", |cg| cg
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
    )
}

