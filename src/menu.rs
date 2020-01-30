use serenity::{
    builder::CreateEmbed,
    model::channel::{Reaction, ReactionType},
    model::id::{ChannelId, MessageId, UserId},
    prelude::*,
};

use crate::core::consts::BOT_ID;
use crate::core::store::{BotOwnerContainer, MessagePagination, MessagePaginator};
use crate::menu::Error::PaginationError;

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
pub enum Error {
    PaginationError,
}

pub type HandlerFunc =
    fn(&Context, &Reaction) -> fn(&Context, ChannelId, MessageId) -> Result<(), Error>;

/// Create a new menu pagination
pub fn new_pagination(
    context: &Context,
    message_id: MessageId,
    author_id: UserId,
    pages: Vec<CreateEmbed>,
) {
    new_pagination_with_handler(context, message_id, author_id, pages, None)
}

/// Create a new menu pagination with a a custom reaction handler
pub fn new_pagination_with_handler(
    context: &Context,
    message_id: MessageId,
    author_id: UserId,
    pages: Vec<CreateEmbed>,
    handler: Option<HandlerFunc>,
) {
    let mut data = context.data.write();
    let paginator = data.get_mut::<MessagePaginator>().unwrap();
    paginator.insert(
        message_id,
        MessagePagination {
            author_id,
            current_page: 0,
            handler,
            message_id,
            pages,
            deleted: false,
        },
    );
}

/// Handles menu reactions
///
/// Triggered by serenity's EventHandler
pub fn handle_reaction(ctx: &Context, reaction: &Reaction) {
    let handler = get_handler(ctx, reaction);

    if handler.is_err() {
        return;
    }

    let func = match handler.unwrap() {
        Some(handler) => handler(ctx, reaction),
        None => match reaction.emoji {
            ReactionType::Unicode(ref x) if x == reactions::NEXT => right,
            ReactionType::Unicode(ref x) if x == reactions::PREV => left,
            ReactionType::Unicode(ref x) if x == reactions::STOP => {
                let delete_reactions = reaction.message(&ctx.http).unwrap().delete_reactions(ctx);

                if delete_reactions.is_ok() {
                    let mut data = ctx.data.write();
                    let paginator = data.get_mut::<MessagePaginator>().unwrap();
                    paginator
                        .entry(reaction.message_id)
                        .and_modify(|pagination| {
                            pagination.pages = vec![];
                            pagination.deleted = true;
                        });
                }

                return;
            }
            _ => return,
        },
    };

    if let Err(why) = func(ctx, reaction.channel_id, reaction.message_id) {
        warn!("Err reacting to reaction: {:?}", why);
    }
}

pub fn left(ctx: &Context, channel_id: ChannelId, message_id: MessageId) -> Result<(), Error> {
    let page = match modify_page(ctx, message_id, &Modifier::Decrement) {
        Some(page) => page,
        None => return Ok(()),
    };

    let embed_content = get_page_content(ctx, message_id, page);

    if let Some(embed_content) = embed_content {
        if let Err(why) = channel_id.edit_message(&ctx.http, message_id, |m| {
            m.embed(|e| {
                e.clone_from(&embed_content);
                e
            })
        }) {
            warn!("Err editing message: {:?}", why);
        }
    }

    Ok(())
}

pub fn right(ctx: &Context, channel_id: ChannelId, message_id: MessageId) -> Result<(), Error> {
    let page = match modify_page(ctx, message_id, &Modifier::Increment) {
        Some(page) => page,
        None => return Ok(()),
    };

    let embed_content = get_page_content(ctx, message_id, page);

    if let Some(embed_content) = embed_content {
        if let Err(why) = channel_id.edit_message(&ctx.http, message_id, |m| {
            m.embed(|e| {
                e.clone_from(&embed_content);
                e
            })
        }) {
            warn!("Err editing message: {:?}", why);
        }
    }

    Ok(())
}

pub fn modify_page(context: &Context, message_id: MessageId, modifier: &Modifier) -> Option<u32> {
    let mut data = context.data.write();

    let paginator = data.get_mut::<MessagePaginator>().unwrap();

    let pagination = match paginator.get_mut(&message_id) {
        Some(pagination) => pagination,
        None => return None,
    };

    match *modifier {
        Modifier::Decrement => {
            if let Some(x) = pagination.current_page.checked_sub(1) {
                pagination.current_page = x;
            } else {
                return None;
            }
        }
        Modifier::Increment => {
            if let Some(x) = pagination.current_page.checked_add(1) {
                pagination.current_page = x;
            } else {
                return None;
            }
        }
    };

    if pagination.current_page as usize > pagination.pages.len() - 1 {
        pagination.current_page -= 1;
        return None;
    }

    Some(pagination.current_page)
}

pub fn get_page_content(
    context: &Context,
    message_id: MessageId,
    page: u32,
) -> Option<CreateEmbed> {
    let data = context.data.read();

    //    let paginator = data.get_mut::<MessagePaginator>().unwrap();
    let paginator = data.get::<MessagePaginator>().unwrap();

    let pagination = match paginator.get(&message_id) {
        Some(pagination) => pagination,
        None => return None,
    };

    Some(pagination.pages[page as usize].clone())
}

fn get_handler(ctx: &Context, reaction: &Reaction) -> Result<Option<HandlerFunc>, Error> {
    let data = ctx.data.read();
    let paginator = data.get::<MessagePaginator>().unwrap();
    let owner = data.get::<BotOwnerContainer>().unwrap();

    let pagination = match paginator.get(&reaction.message_id) {
        Some(_pagination) => _pagination,
        None => return Err(PaginationError),
    };

    let is_paginated_msg = pagination.message_id == reaction.message_id;
    let is_current_bot = &BOT_ID == reaction.user_id.as_u64();
    let is_author = pagination.author_id == reaction.user_id;
    let is_owner = owner.id == reaction.user_id;

    if !is_current_bot {
        match reaction.delete(ctx) {
            Ok(_) => (),
            Err(why) => warn!("Err deleting reaction: {:?}", why),
        }
    }

    if !(is_paginated_msg && !is_current_bot && (is_author || is_owner)) {
        return Err(PaginationError);
    }

    Ok(pagination.handler)
}
