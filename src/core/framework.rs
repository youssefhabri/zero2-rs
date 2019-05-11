use std::collections::HashSet;
use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;

use crate::core::consts::PREFIX;
use crate::commands::{self, anilist, fun, meta, nekoslife, urban};

pub struct Zero2Framework;

impl Zero2Framework {
    pub fn with_owners(owners: HashSet<UserId>) -> StandardFramework {
        StandardFramework::new()
            .configure(|c| c
                .with_whitespace(true)
                .allow_dm(true)
                .on_mention(true)
                .ignore_bots(true)
                .case_insensitivity(true)
                .delimiters(vec![",", " "])
                .owners(owners)
                .prefix(PREFIX.as_str())
            )
            .before(|ctx, msg, _| {
                let _ = msg.channel_id.broadcast_typing(&ctx.http);
                true
            })
            .bucket("stats_limit", |b| b.delay(6 * 3600))
            .help(&commands::ZERO2_HELP_HELP_COMMAND)
            .group(&anilist::ANILIST_GROUP)
            .group(&urban::KNOWLEDGE_GROUP)
            .group(&fun::FUN_GROUP)
            .group(&meta::META_GROUP)
            .group(&nekoslife::NEKOSLIFE_GROUP)
            .group(&commands::NO_CATEGORY_GROUP)
    }
}