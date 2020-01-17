use serenity::framework::standard::{Args, Delimiter, StandardFramework};
use serenity::model::id::{GuildId, UserId};
use std::collections::HashSet;

use crate::commands::{self, anilist, fun, meta, nekoslife, profile, system, urban};
use crate::core::cc_parser;
use crate::core::consts::{BOT_ID, DB, PREFIX, PREFIXES};
use crate::core::store::{Command, CommandLogger};
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
                    .dynamic_prefix(|_ctx, msg| {
                        if let Some(guild_id) = msg.guild_id {
                            if let Ok(guild) = DB.find_guild(guild_id) {
                                if !guild.prefix.is_empty() {
                                    return Some(guild.prefix);
                                }
                            }
                        }

                        None
                    })
            })
            .before(|ctx, msg, cmd| {
                // TODO check blacklisted commands & users

                if cmd != "shutdown" {
                    let _ = msg.channel_id.broadcast_typing(&ctx.http);
                }

                {
                    let mut data = ctx.data.write();
                    let cmd_logger = data.get_mut::<CommandLogger>().unwrap();
                    cmd_logger.insert(
                        msg.id,
                        Command {
                            guild_id: msg.guild_id.unwrap_or(GuildId(0)),
                            name: cmd.to_string(),
                            message: msg.content.clone(),
                            user_id: msg.author.id,
                            time: msg.timestamp,
                        },
                    );
                }

                true
            })
            .unrecognised_command(|ctx, message, cmd| {
                if let Ok(command) = DB.find_command(cmd.to_string()) {
                    if Some(GuildId(command.guild_id as u64)) != message.guild_id {
                        return;
                    }

                    match command.kind.as_str() {
                        "text" => {
                            let _ = message
                                .channel_id
                                .send_message(ctx, |m| m.content(command.content));
                        }
                        "simple_parsable" => {
                            let args_raw = message
                                .content
                                .replace(format!("{}{}", PREFIX.as_str(), cmd).as_str(), "");
                            let args = Args::new(args_raw.as_str(), &[Delimiter::Single(' ')]);

                            cc_parser::parse(ctx, message, &args, command.content);
                        }
                        _ => {}
                    }
                }
            })
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
            .group(&profile::PROFILE_GROUP)
            .group(&commands::NOCATEGORY_GROUP)
    }
}
