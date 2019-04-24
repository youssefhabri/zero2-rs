use serenity::framework::standard::CreateGroup;


mod avatar;
mod botinfo;
mod ping;
mod stats;
mod test;

pub fn init_meta() -> CreateGroup {
    CreateGroup::default()
        .command("ping", |c| c
            .cmd(ping::Ping)
        )
        .command("info", |c| c
            .cmd(botinfo::BotInfo)
        )
        .command("stats", |c| c
            .cmd(stats::Stats)
            .bucket("stats_limit")
        )
        .command("avatar", |c| c
            .cmd(avatar::Avatar)
        )
        .command("test", |c| c
            .cmd(test::Test)
        )
}
