use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::prelude::*;
use serenity::model::channel::Message;

use indexmap::IndexMap;
use serenity::model::id::UserId;
use serenity::builder::CreateEmbed;
use crate::core::consts::AT_BOT_IDS;


#[command("stats")]
fn stats_command(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let sending = message.channel_id.say(
        &context.http,
        "_Crunching numbers, please be patient ..._"
    );

    let messages = get_all_messages(&context, message);

    match messages {
        Some(messages) => {
            let mut stats: IndexMap<UserId, u32> = IndexMap::new();

            for msg in messages {
                let author_id = msg.author.id;
                match stats.get(&author_id) {
                    Some(count) => stats.insert(author_id, count + 1),
                    None => stats.insert(author_id, 1),
                };
            }

            stats.sort_by(|_, a, _, b| b.cmp(a));

            let _ = sending.unwrap().delete(&context);

            let _ = message.channel_id.send_message(&context.http,
                |m| m
                    .embed(|embed| build_embed(
                        embed, message.channel_id.name(&context), stats))
            );
        },
        None => {
            let _ = message.channel_id.say(&context.http, "Error getting the channel messages.");
        }
    }

    Ok(())
}

fn get_all_messages(context: &Context, message: &Message) -> Option<Vec<Message>> {
    let mut all_messages: Vec<Message> = vec![];

    let limit = 100;

    let mut messages = message.channel_id.messages(
        &context.http,
        |g| g.before(message.id).limit(limit)
    ).unwrap();

    while !messages.is_empty() {
        all_messages.extend(messages.clone());
        let last_message_id = messages[messages.len() - 1].id;
        messages = message.channel_id.messages(
            &context.http,
            |g| g.before(last_message_id).limit(limit)
        ).unwrap();
    }

    if !all_messages.is_empty() { Some(all_messages) }
    else { None }
}

fn build_embed(embed: &mut CreateEmbed, channel_name: Option<String>, stats_list: IndexMap<UserId, u32>) -> &mut CreateEmbed {

    let content = stats_list
        .iter()
        .take(10)
        .enumerate()
        .filter(|(_, (user_id, _))| !AT_BOT_IDS.contains(user_id.as_u64()))
        .map(|(i, (user_id, msgs_count))|{
            format!("{}. <@{}>: {}", i + 1, user_id, msgs_count)
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    embed
        .title(format!("Messages stats in #{}", channel_name.unwrap()))
        .description(content)
}