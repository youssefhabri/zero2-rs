use serenity::framework::standard::CreateGroup;

mod golendar;
mod fortune;

pub fn init_fun() -> CreateGroup {
    CreateGroup::default()
        .command("fortune", |c| c
            .cmd(fortune::FortuneCommand)
            .desc("Find out you fortune. It just might be you lucky day ...")
        )
        .command("golendar", |c| c
            .cmd(golendar::GolendarCommand)
            .batch_known_as(vec!["gol"])
            .desc("Find out you fortune. It just might be you lucky day ...")
        )
}
