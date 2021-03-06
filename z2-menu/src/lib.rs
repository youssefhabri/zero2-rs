#[macro_use]
extern crate log;

use serenity::model::id::UserId;
use serenity::model::prelude::{Reaction, ReactionType};
use serenity::prelude::Context;

use crate::types::PaginationContainer;

pub mod anilist;
pub mod giphy;
pub mod urban;

pub mod reactions;
pub mod types;
pub mod utils;

pub async fn handle_reaction(context: &Context, reaction: &Reaction) {
    // TODO fix scoping to limit the time the lock is acquired
    //  This should not be an issue for Zero Two as its not used in a lot of servers,
    //  but it would be better to fix this anyways.
    //
    //  One way to fix this is to aquire a read-only lock, act on the data, and then
    //   aquire a write lock and update the data. This however my cause a race condition,
    //   if two events try to act on the same pagination.
    //
    //  Maybe wait till we have a caching anilist client, and then benchmark it and see how
    //   long does it take for the lock to be released
    let mut data = context.data.write().await;

    // Maybe after acquiring the Pagination lock, release the context.data lock? is that even possible?
    let mut container = match data.get_mut::<PaginationContainer>() {
        Some(container) => container.write().await,
        None => return,
    };

    let pagination_info = match container.get_mut(&reaction.message_id) {
        Some(pagination_info) => pagination_info,
        None => return,
    };

    match reaction.delete(&context).await {
        Ok(_) => (),
        Err(why) => error!("Err deleting reaction: {:?}", why),
    }

    let is_author = reaction.user_id == Some(pagination_info.author);
    let is_owner = reaction.user_id == Some(UserId(139360031102599168));
    if !(is_author || is_owner) || pagination_info.ended {
        return;
    }

    let pagination = &mut pagination_info.pagination;

    match reaction.emoji {
        ReactionType::Unicode(ref x) if x == reactions::NEXT => {
            let cursor = (pagination.cursor() + 1).min(pagination.count() - 1);
            pagination.set_cursor(cursor);
        }

        ReactionType::Unicode(ref x) if x == reactions::PREV => {
            let cursor = pagination.cursor().saturating_sub(1);
            pagination.set_cursor(cursor);
        }

        ReactionType::Unicode(ref x) if x == reactions::FIRST => {
            pagination.set_cursor(0);
        }

        ReactionType::Unicode(ref x) if x == reactions::LAST => {
            let cursor = pagination.count() - 1;
            pagination.set_cursor(cursor);
        }

        ReactionType::Unicode(ref x) if x == reactions::STOP => {
            let delete_reactions = context
                .http
                .delete_message_reactions(
                    *reaction.channel_id.as_u64(),
                    *reaction.message_id.as_u64(),
                )
                .await;

            if delete_reactions.is_ok() {
                pagination_info.ended = true;
            }

            // TODO should we remove the message from the store?

            return;
        }
        ReactionType::Unicode(ref x) if x == reactions::DELETE => {
            let message_id = reaction.message_id;
            let content = format!(
                ":no_entry: The {} embed has been deleted!",
                pagination.name()
            );
            let edit_message = reaction
                .channel_id
                .edit_message(&context, message_id, |m| {
                    m.content(content).suppress_embeds(true)
                })
                .await;

            if let Ok(msg) = edit_message {
                // pagination_info.ended = true;
                let _ = msg.delete_reactions(&context).await;
                let _ = container.remove(&msg.id);
            }

            return;
        }

        _ => {}
    }

    if let Err(why) = pagination.handle(&context, &reaction).await {
        error!("{}", why);
    }
}
