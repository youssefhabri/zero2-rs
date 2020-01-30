use indexmap::IndexMap;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::*;

use crate::core::consts::AT_BOT_IDS;

#[command]
#[bucket = "stats_limit"]
fn stats(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let sending = message
        .channel_id
        .say(&context.http, "_Crunching numbers, please be patient ..._");

    let messages = get_all_messages(&context, message);

    match messages {
        Some(messages) => {
            let mut stats: IndexMap<UserId, u32> = IndexMap::new();

            for msg in messages {
                stats
                    .entry(msg.author.id)
                    .and_modify(|v| v.clone_from(&(*v + 1)))
                    .or_insert(1);
            }

            stats.sort_by(|_, a, _, b| b.cmp(a));

            let _ = sending?.delete(&context);

            let _ = message.channel_id.send_message(&context.http, |m| {
                m.embed(|embed| build_embed(embed, message.channel_id.name(&context), stats))
            })?;
        }
        None => {
            let _ = message
                .channel_id
                .say(&context.http, "Error getting the channel messages.")?;
        }
    }

    Ok(())
}

fn get_all_messages(context: &Context, message: &Message) -> Option<Vec<Message>> {
    let mut all_messages: Vec<Message> = vec![];

    let limit = 100;

    let mut messages = message
        .channel_id
        .messages(&context.http, |g| g.before(message.id).limit(limit))
        .unwrap();

    while !messages.is_empty() {
        all_messages.extend(messages.clone());
        let last_message_id = messages[messages.len() - 1].id;
        messages = message
            .channel_id
            .messages(&context.http, |g| g.before(last_message_id).limit(limit))
            .unwrap();
    }

    if !all_messages.is_empty() {
        Some(all_messages)
    } else {
        None
    }
}

fn build_embed(
    embed: &mut CreateEmbed,
    channel_name: Option<String>,
    stats_list: IndexMap<UserId, u32>,
) -> &mut CreateEmbed {
    let content = stats_list
        .iter()
        .take(10)
        .enumerate()
        .filter(|(_, (user_id, _))| !AT_BOT_IDS.contains(user_id.as_u64()))
        .map(|(i, (user_id, msgs_count))| format!("{}. <@{}>: {}", i + 1, user_id, msgs_count))
        .collect::<Vec<String>>()
        .join("\n\n");

    embed
        .title(format!("Messages stats in #{}", channel_name.unwrap()))
        .description(content)
}
