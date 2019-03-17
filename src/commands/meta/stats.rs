use serenity::framework::standard::{Args, Command, CommandError};
use serenity::prelude::*;
use serenity::model::channel::Message;

use indexmap::IndexMap;
use serenity::model::id::{UserId, ChannelId};
use serenity::builder::CreateEmbed;

pub struct Stats;

impl Command for Stats {
    fn execute(&self, _: &mut Context, message: &Message, _: Args) -> Result<(), CommandError> {
        let sending = message.channel_id.say("_Crunching numbers, please be patient ..._");

        let messages = get_all_messages(message);

        match messages {
            Some(messages) => {
                let mut stats: IndexMap<UserId, u32> = IndexMap::new();

                for msg in messages {
                    let author_id = msg.author.id;
                    let count = stats.get(&author_id);
                    if count.is_some() {
                        stats.insert(author_id, count.unwrap() + 1);
                    } else {
                        stats.insert(author_id, 1);
                    }

                    if stats.len() >= 10 { break; }
                }

                stats.sort_by(|_, a, _, b| b.cmp(a));

                let _ = sending.unwrap().delete();

                let _ = message.channel_id.send_message(|m| m
                    .embed(|_| build_embed(message.channel_id, stats))
                );
            },
            None => {
                let _ = message.channel_id.say("Error getting the channel messages.");
            }
        }

        Ok(())
    }
}

fn get_all_messages(message: &Message) -> Option<Vec<Message>> {
    let mut all_messages: Vec<Message> = vec![];

    let limit = 100;

    let mut messages = message.channel_id.messages(|g| g
        .before(message.id).limit(limit)
    ).unwrap();

    while messages.len() > 0 {
        all_messages.extend(messages.clone());
        let last_message_id = messages[messages.len() - 1].id;
        messages = message.channel_id.messages(|g| g
            .before(last_message_id).limit(limit)
        ).unwrap();
    }

    if all_messages.len() > 0 { Some(all_messages) }
    else { None }
}

fn build_embed(channel_id: ChannelId, stats_list: IndexMap<UserId, u32>) -> CreateEmbed {

    let mut content = String::new();

    for (i, (user_id, msg_number)) in stats_list.iter().enumerate() {
        content = format!("{}\n\n{}. <@{}>: {}", content, i + 1, user_id, msg_number);
    }

    CreateEmbed::default()
        .title(format!("Messages stats in #{}", channel_id.name().unwrap()))
        .description(content)
}