use chrono::{Date, Datelike, Duration, Utc, Weekday};
use serenity::builder::CreateEmbed;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message, ReactionType};
use serenity::model::id::{MessageId, UserId};
use serenity::prelude::Context;
use std::ops::Add;

use crate::reactions;
use crate::types::{Pagination, PaginationContainer, PaginationInfo};

pub async fn send_embed_message(
    context: &Context,
    message: &Message,
    embed: &CreateEmbed,
    reactions: Vec<ReactionType>,
) -> CommandResult<Message> {
    let sent = message
        .channel_id
        .send_message(&context, |m| {
            m.embed(|e| {
                e.clone_from(embed);
                e
            })
            .reactions(reactions)
        })
        .await?;

    Ok(sent)
}

pub async fn add_pagination_to_store<P>(
    context: &Context,
    pagination: P,
    message_id: MessageId,
    author_id: UserId,
) where
    P: Pagination + 'static,
{
    let data = context.data.write().await;
    let container = data.get::<PaginationContainer>().unwrap();
    let pagination_info = PaginationInfo::new(author_id, pagination);
    container.write().await.insert(message_id, pagination_info);
}

pub fn num_to_emoji(num: u32) -> String {
    match num {
        0 => ":zero:",
        1 => ":one:",
        2 => ":two:",
        3 => ":three:",
        4 => ":four:",
        5 => ":five:",
        6 => ":six:",
        7 => ":seven:",
        8 => ":eight:",
        9 => ":nine:",
        _ => unreachable!("Input should not be a number above 9."),
    }
    .to_string()
}

pub fn reaction_to_weekday(reaction: &str) -> Option<Weekday> {
    match reaction {
        reactions::ONE => Some(Weekday::Mon),
        reactions::TWO => Some(Weekday::Tue),
        reactions::THREE => Some(Weekday::Wed),
        reactions::FOUR => Some(Weekday::Thu),
        reactions::FIVE => Some(Weekday::Fri),
        reactions::SIX => Some(Weekday::Sat),
        reactions::SEVEN => Some(Weekday::Sun),
        _ => None,
    }
}

pub fn weekday_to_date(weekday: Weekday) -> Date<Utc> {
    let mut date = Utc::today();
    while date.weekday() != weekday {
        date = date.add(Duration::days(1));
    }

    date
}
