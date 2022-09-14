use serenity::framework::standard::macros::{help, hook};
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;
use std::collections::HashSet;

use super::consts::{PREFIX, PREFIXES};
use crate::commands::anilist::ANILIST_GROUP;
use crate::commands::fun::FUN_GROUP;
use crate::commands::knowledge::KNOWLEDGE_GROUP;
use crate::commands::meta::META_GROUP;
use crate::commands::system::SYSTEM_GROUP;
use crate::commands::ROOT_GROUP;

#[cfg(feature = "db")]
use crate::commands::config::CONFIGURATION_GROUP;

pub struct Zero2Framework;

impl Zero2Framework {
    pub fn with_info(owners: HashSet<UserId>, bot_id: Option<UserId>) -> StandardFramework {
        let framework = StandardFramework::new()
            .configure(|c| {
                c.with_whitespace(true)
                    .allow_dm(true)
                    .on_mention(bot_id)
                    .ignore_bots(true)
                    .delimiters(vec![",", " "])
                    .owners(owners)
                    .prefix(PREFIX.as_str())
                    .prefixes(PREFIXES.to_vec())
            })
            .before(before)
            .after(after)
            .help(&MY_HELP)
            .group(&ROOT_GROUP)
            .group(&ANILIST_GROUP)
            .group(&FUN_GROUP)
            .group(&KNOWLEDGE_GROUP)
            .group(&META_GROUP)
            .group(&SYSTEM_GROUP);

        #[cfg(feature = "db")]
        framework.group(&CONFIGURATION_GROUP);

        framework
    }
}

#[hook]
async fn before(_context: &Context, _message: &Message, _command_name: &str) -> bool {
    true
}

#[hook]
async fn after(
    _context: &Context,
    _msg: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    if let Err(why) = command_result {
        let error = format!("Error in {}: {}", command_name, why);
        error!("{}", &error);
        let _ = _msg.channel_id.say(&_context, error).await;
    }
}

// TODO change the help message

#[help]
#[individual_command_tip = "Hello! こんにちは！Hola! Bonjour! 您好!\n\
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
