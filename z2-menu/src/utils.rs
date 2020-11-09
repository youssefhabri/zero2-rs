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
                e.clone_from(&embed);
                e
            })
            .reactions(reactions)
        })
        .await?;

    Ok(sent)
}

pub async fn add_pagination_to_store(
    context: &Context,
    pagination: Box<dyn Pagination>,
    message_id: MessageId,
    author_id: UserId,
) {
    let data = context.data.write().await;
    let container = data.get::<PaginationContainer>().unwrap();
    let pagination_info = PaginationInfo::new(author_id, pagination);
    container.lock().await.insert(message_id, pagination_info);
}

pub fn num_to_emoji(num: u32) -> String {
    match num {
        0 => ":zero:".to_string(),
        1 => ":one:".to_string(),
        2 => ":two:".to_string(),
        3 => ":three:".to_string(),
        4 => ":four:".to_string(),
        5 => ":five:".to_string(),
        6 => ":six:".to_string(),
        7 => ":seven:".to_string(),
        8 => ":eight:".to_string(),
        9 => ":nine:".to_string(),
        _ => num.to_string(),
    }
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
