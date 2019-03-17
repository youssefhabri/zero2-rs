use serenity::framework::StandardFramework;


mod botinfo;
mod ping;
mod stats;

pub fn register(framework: StandardFramework) -> StandardFramework {
    framework.group("Meta", |cg| cg
        .command("ping", |c| c
            .cmd(ping::Ping)
        )
        .command("info", |c| c
            .cmd(botinfo::BotInfo)
        )
        .command("stats", |c| c
            .cmd(stats::Stats)
        )
    )
}
