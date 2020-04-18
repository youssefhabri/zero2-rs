use serenity::framework::standard::StandardFramework;
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;
use std::collections::HashSet;

use super::consts::{BOT_ID, PREFIX, PREFIXES};
use super::utils;
use crate::commands::{self, anilist, fun, meta, nekoslife, system, urban};
use crate::monitors;

pub struct Zero2Framework;

impl Zero2Framework {
    pub fn with_owners(owners: HashSet<UserId>) -> StandardFramework {
        StandardFramework::new()
            .configure(|c| {
                c.with_whitespace(true)
                    .allow_dm(true)
                    .on_mention(Some(UserId(BOT_ID)))
                    .ignore_bots(true)
                    .case_insensitivity(true)
                    .delimiters(vec![",", " "])
                    .owners(owners)
                    .prefix(PREFIX.as_str())
                    .prefixes(PREFIXES.to_vec())
            })
            .before(before)
            .normal_message(|ctx, msg| {
                monitors::message_monitors(ctx, msg);
            })
            .bucket("stats_limit", |b| b.delay(6 * 3600))
            .help(&commands::ZERO2_HELP)
            .group(&anilist::ANILIST_GROUP)
            .group(&urban::KNOWLEDGE_GROUP)
            .group(&fun::FUN_GROUP)
            .group(&meta::META_GROUP)
            .group(&nekoslife::NEKOSLIFE_GROUP)
            .group(&system::SYSTEM_GROUP)
            .group(&commands::NOCATEGORY_GROUP)
    }
}

fn before(ctx: &mut Context, msg: &Message, cmd: &str) -> bool {
    if cmd != "shutdown" {
        let _ = msg.channel_id.broadcast_typing(&ctx.http);
    }

    utils::log_command(ctx, msg, cmd);

    true
}
