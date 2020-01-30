use serenity::model::channel::{Reaction, ReactionType};
use serenity::model::id::{ChannelId, MessageId};
use serenity::prelude::Context;

use crate::menu::{reactions, utils, Error, HandlerFuncReturn};

// TODO investigate the possibility to generalize the menu system
//pub fn static_pages_handler(
//    context: &Context,
//    reaction: &Reaction,
//) -> fn(&Context, ChannelId, MessageId) -> Result<(), Error> {
//    let ReactionType::Unicode(ref emoji) = reaction.emoji;
//
//    let reactions: Vec<String> = reaction
//        .message(&context)
//        .unwrap()
//        .reactions
//        .into_iter()
//        .map(|react| react.reaction_type.as_data())
//        .collect::<Vec<String>>();
//
//    let page_index = reactions.into_iter().position(|em| em == emoji);
//}

const ANIME_STATS_PAGE: u32 = 0;
const MANGA_STATS_PAGE: u32 = 1;

pub fn user_stats_handler(context: &Context, reaction: &Reaction) -> HandlerFuncReturn {
    match reaction.emoji {
        ReactionType::Unicode(ref x) if x == reactions::ANIME => Some(show_anime_stats),
        ReactionType::Unicode(ref x) if x == reactions::MANGA => Some(show_manga_stats),
        ReactionType::Unicode(ref x) if x == reactions::STOP => {
            utils::stop_pagination(&context, &reaction);

            None
        }
        _ => None,
    }
}

fn show_anime_stats(
    context: &Context,
    channel_id: ChannelId,
    message_id: MessageId,
) -> Result<(), Error> {
    let page_content = utils::get_page_content(context, message_id, ANIME_STATS_PAGE);
    utils::update_message(&context, channel_id, message_id, page_content);

    Ok(())
}

fn show_manga_stats(
    context: &Context,
    channel_id: ChannelId,
    message_id: MessageId,
) -> Result<(), Error> {
    let page_content = utils::get_page_content(context, message_id, MANGA_STATS_PAGE);
    utils::update_message(&context, channel_id, message_id, page_content);

    Ok(())
}
