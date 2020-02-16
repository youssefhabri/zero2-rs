use serenity::{
    builder::CreateEmbed,
    model::channel::{Reaction, ReactionType},
    model::id::{ChannelId, MessageId, UserId},
    prelude::*,
};

use crate::core::consts::BOT_ID;
use crate::core::store::{BotOwnerContainer, MessagePagination, MessagePaginator, PaginationKind};
use crate::menu::Error::PaginationError;

pub mod builders;
pub mod handlers;
pub mod utils;

/// Emulating an enum of reactions ¯\_(ツ)_/¯
pub mod reactions;

/// Menu modifier enum
///
/// Used to tell the menu system whether to go forward or backward
pub enum Modifier {
    Decrement,
    Increment,
    First,
    Last,
}

#[derive(Debug)]
pub enum Error {
    PaginationError,
}

pub type HandlerFunc = fn(&Context, &Reaction) -> HandlerFuncReturn;
pub type HandlerFuncReturn = Option<fn(&Context, ChannelId, MessageId) -> Result<(), Error>>;

/// Create a new menu pagination
pub fn new_pagination(
    context: &Context,
    message_id: MessageId,
    author_id: UserId,
    kind: PaginationKind,
    pages: Vec<String>,
) {
    new_pagination_with_handler(context, message_id, author_id, kind, pages, None)
}

/// Create a new menu pagination with a a custom reaction handler
pub fn new_pagination_with_handler(
    context: &Context,
    message_id: MessageId,
    author_id: UserId,
    kind: PaginationKind,
    pages: Vec<String>,
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
            kind,
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

    // Delete user reactions except the initial bot reactions
    if !reaction.user_id.as_u64() != BOT_ID {
        match reaction.delete(ctx) {
            Ok(_) => (),
            Err(why) => error!("Err deleting reaction: {:?}", why),
        }
    }

    let result = match handler.unwrap() {
        Some(handler) => match handler(ctx, reaction) {
            Some(handler_func) => handler_func(&ctx, reaction.channel_id, reaction.message_id),
            None => return,
        },
        None => default_handler(&ctx, &reaction),
    };

    if let Err(why) = result {
        warn!("Err reacting to reaction: {:?}", why);
    }
}

pub fn default_handler(context: &Context, reaction: &Reaction) -> Result<(), Error> {
    let channel_id = reaction.channel_id;
    let message_id = reaction.message_id;

    match reaction.emoji {
        ReactionType::Unicode(ref x) if x == reactions::NEXT => {
            update_message(&context, channel_id, message_id, &Modifier::Increment)
        }
        ReactionType::Unicode(ref x) if x == reactions::PREV => {
            update_message(&context, channel_id, message_id, &Modifier::Decrement)
        }
        ReactionType::Unicode(ref x) if x == reactions::FIRST => {
            update_message(&context, channel_id, message_id, &Modifier::First)
        }
        ReactionType::Unicode(ref x) if x == reactions::LAST => {
            update_message(&context, channel_id, message_id, &Modifier::Last)
        }
        ReactionType::Unicode(ref x) if x == reactions::STOP => {
            utils::stop_pagination(&context, &reaction);

            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn update_message(
    context: &Context,
    channel_id: ChannelId,
    message_id: MessageId,
    modifier: &Modifier,
) -> Result<(), Error> {
    update_current_page(context, message_id, modifier)?;

    let embed_content = get_page_content(context, message_id);
    utils::update_message(context, channel_id, message_id, embed_content);

    Ok(())
}

pub fn update_current_page(
    context: &Context,
    message_id: MessageId,
    modifier: &Modifier,
) -> Result<(), Error> {
    let mut data = context.data.write();

    let paginator = data.get_mut::<MessagePaginator>().unwrap();

    let pagination = match paginator.get_mut(&message_id) {
        Some(pagination) => pagination,
        None => return Err(Error::PaginationError),
    };

    let new_page = match *modifier {
        Modifier::Decrement => pagination.current_page.saturating_sub(1),
        Modifier::Increment => (pagination.current_page + 1).min(pagination.pages.len() - 1),
        Modifier::First => 0,
        Modifier::Last => pagination.pages.len() - 1,
    };

    if new_page != pagination.current_page {
        pagination.current_page = new_page;
    }

    Ok(())
}

// TODO untangle this mess, pls?
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

    if !(is_paginated_msg && !is_current_bot && (is_author || is_owner)) {
        return Err(PaginationError);
    }

    Ok(pagination.handler)
}

pub fn get_page_content(context: &Context, message_id: MessageId) -> Option<CreateEmbed> {
    let data = context.data.read();

    //    let paginator = data.get_mut::<MessagePaginator>().unwrap();
    let paginator = data.get::<MessagePaginator>().unwrap();

    paginator
        .get(&message_id)
        .and_then(|pagination| get_page_builder(pagination))
}

fn get_page_builder(pagination: &MessagePagination) -> Option<CreateEmbed> {
    let data = pagination.pages[pagination.current_page].clone();
    let prefix = format!(
        "Page: {}/{} | ",
        pagination.current_page + 1,
        pagination.pages.len()
    );

    match pagination.kind {
        // AniList
        PaginationKind::Media => serde_json::from_str(data.as_str())
            .ok()
            .map(|media| builders::media_embed_builder(&media, prefix)),
        PaginationKind::Character => serde_json::from_str(data.as_str())
            .ok()
            .map(|character| builders::character_embed_builder(&character, prefix)),
        PaginationKind::Staff => serde_json::from_str(data.as_str())
            .ok()
            .map(|staff| builders::staff_embed_builder(&staff, prefix)),
        PaginationKind::User => serde_json::from_str(data.as_str())
            .ok()
            .map(|user| builders::user_embed_builder(&user, prefix)),
        PaginationKind::Urban => serde_json::from_str(data.as_str())
            .ok()
            .map(|definition| builders::urban_embed_builder(&definition, prefix)),
        PaginationKind::Giphy => serde_json::from_str(data.as_str())
            .ok()
            .map(|gif| builders::giphy_embed_builder(&gif, prefix)),
    }
}
