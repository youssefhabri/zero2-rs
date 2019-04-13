use serenity::{
    prelude::*,
    client::CACHE,
    model::channel::{Reaction, ReactionType},
    model::id::{ChannelId, MessageId, UserId},
    builder::CreateEmbed
};

use crate::store::{BotOwnerContainer, MessagePaginator, MessagePagination};

pub mod builders;

/// Emulating an enum of reactions ¯\_(ツ)_/¯
pub mod reactions {
    pub const PREV: &str = "⬅";
    pub const NEXT: &str = "➡";
    pub const STOP: &str = "🇽";

    pub fn default<'a>() -> Vec<&'a str> {
        [PREV, NEXT, STOP].to_vec()
    }
}

/// Menu modifier enum
/// 
/// Used to tell the menu system whether to go forward or backward
pub enum Modifier {
    Decrement,
    Increment,
}

#[derive(Debug)]
pub enum Error {}

pub type HandlerFunc = fn(&Context, &Reaction) -> fn(&Context, ChannelId, MessageId) -> Result<(), Error>;

/// Create a new menu pagination
pub fn new_pagination(context: &Context, message_id: MessageId, author_id: UserId, pages: Vec<CreateEmbed>) {
    new_pagination_with_handler(
        context,
        message_id,
        author_id,
        pages,
        None
    )
}

/// Create a new menu pagination with a a custom reaction handler
pub fn new_pagination_with_handler(
    context: &Context,
    message_id: MessageId,
    author_id: UserId,
    pages: Vec<CreateEmbed>,
    handler: Option<HandlerFunc>
) {
    let mut data = context.data.lock();
    let paginator = data.get_mut::<MessagePaginator>().unwrap();
    paginator.insert(message_id, MessagePagination {
        author_id,
        current_page: 0,
        handler,
        message_id,
        pages,
    });
}

/// Hanldes menu reactions
/// 
/// Triggered by serenity's EventHandler
pub fn handle_reaction(ctx: &Context, reaction: &Reaction) {
    let cache = CACHE.read();

    let handler = if let Some(_channel) = cache.channels.get(&reaction.channel_id) {

        let data = ctx.data.lock();
        let paginator = data.get::<MessagePaginator>().unwrap();
        let owner = data.get::<BotOwnerContainer>().unwrap();

        let pagination = match paginator.get(&reaction.message_id) {
            Some(pagination) => pagination,
            None => return,
        };

        let is_paginated_msg = pagination.message_id == reaction.message_id;
        let is_current_bot = cache.user.id == reaction.user_id;
        let is_author = pagination.author_id == reaction.user_id;
        let is_owner = owner.id == reaction.user_id;

        if !is_current_bot {
            match reaction.delete() {
                Ok(_) => (),
                Err(why) => warn!("Err deleting reaction: {:?}", why)
            }
        }

        if !(is_paginated_msg && !is_current_bot && (is_author || is_owner)) {
            return;
        }

        pagination.handler

    } else {
        return;
    };

    let func = match handler {
        Some(handler) => handler(ctx, reaction),
        None => match reaction.emoji {
            ReactionType::Unicode(ref x) if x == reactions::NEXT => right,
            ReactionType::Unicode(ref x) if x == reactions::PREV => left,
            ReactionType::Unicode(ref x) if x == reactions::STOP => {
                let _ = reaction.message().unwrap().delete_reactions();

                return;
            },
            _ => return,
        }
    };

    if let Err(why) = func(ctx, reaction.channel_id, reaction.message_id) {
        warn!("Err reacting to reaction: {:?}", why);
    }
}

pub fn left(ctx: &Context, channel_id: ChannelId, message_id: MessageId) -> Result<(), Error> {
    let page = match modify_page(ctx, &message_id, &Modifier::Decrement) {
        Some(page) => page,
        None => return Ok(())
    };

    let embed_content = get_page_content(ctx, &message_id, page);

    if let Some(embed_content) = embed_content {
        if let Err(why) = channel_id.edit_message(message_id, |m| m.embed(|_| embed_content)) {
            warn!("Err editing message: {:?}", why);
        }
    }

    Ok(())
}

pub fn right(ctx: &Context, channel_id: ChannelId, message_id: MessageId) -> Result<(), Error> {
    let page = match modify_page(ctx, &message_id, &Modifier::Increment) {
        Some(page) => page,
        None => return Ok(())
    };

    let embed_content = get_page_content(ctx, &message_id, page);

    if let Some(embed_content) = embed_content {
        if let Err(why) = channel_id.edit_message(message_id, |m| m.embed(|_| embed_content)) {
            warn!("Err editing message: {:?}", why);
        }
    }

    Ok(())
}

pub fn modify_page(context: &Context, message_id: &MessageId, modifier: &Modifier) -> Option<u32> {
    let mut data = context.data.lock();

    let paginator = data.get_mut::<MessagePaginator>().unwrap();

    let pagination = match paginator.get_mut(message_id) {
        Some(pagination) => pagination,
        None => return None,
    };

    match *modifier {
        Modifier::Decrement => if let Some(x) = pagination.current_page.checked_sub(1) {
            pagination.current_page = x;
        } else { return None },
        Modifier::Increment => if let Some(x) = pagination.current_page.checked_add(1) {
            pagination.current_page = x;
        } else { return None },
    };

    Some(pagination.current_page)
}

pub fn get_page_content(context: &Context, message_id: &MessageId, page: u32) -> Option<CreateEmbed> {
    let mut data = context.data.lock();

    let paginator = data.get_mut::<MessagePaginator>().unwrap();

    let pagination = match paginator.get_mut(message_id) {
        Some(pagination) => pagination,
        None => return None,
    };

    Some(pagination.pages[page as usize].clone())
}