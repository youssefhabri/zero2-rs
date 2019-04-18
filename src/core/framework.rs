use std::collections::HashSet;
use serenity::framework::standard::{StandardFramework, help_commands, HelpBehaviour};
use serenity::model::id::UserId;

use crate::core::consts::PREFIX;
use crate::commands::{self, anilist, fun, meta, nekoslife, urban};

pub struct Zero2Framework;

impl Zero2Framework {
    pub fn with_owners(owners: HashSet<UserId>) -> StandardFramework {
        StandardFramework::new()
            .customised_help(help_commands::with_embeds, |c| c
                .individual_command_tip("Hello! こんにちは！Hola! Bonjour! 您好!\n\
                    If you want more information about a specific command, just pass the command as argument.")
                .command_not_found_text("Could not find: `{}`.")
                .max_levenshtein_distance(3)
                .lacking_permissions(HelpBehaviour::Hide)
                .lacking_role(HelpBehaviour::Nothing)
                .wrong_channel(HelpBehaviour::Strike)
            )
            .configure(|c| c
                .allow_whitespace(true)
                .allow_dm(true)
                .on_mention(true)
                .ignore_bots(true)
                .case_insensitivity(true)
                .delimiters(vec![",", " "])
                .owners(owners)
                .prefix(PREFIX.as_str())
            )
            .before(|_, msg, _| { let _ = msg.channel_id.broadcast_typing(); true })
            .simple_bucket("stats_limit", 6 * 3600)
            .group("AniList",     |_| anilist::init_anilist())
            .group("Knowledge",   |_| urban::init_knowledge())
            .group("Fun",         |_| fun::init_fun())
            .group("Meta",        |_| meta::init_meta())
            .group("Neko's Life", |_| nekoslife::init_nekoslife())
            .group("No Category", |_| commands::init_no_category())
    }
}