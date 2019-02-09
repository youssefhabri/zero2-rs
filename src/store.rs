use std::sync::Arc;
use std::collections::HashMap;

use serenity::{
    builder::CreateEmbed,
    client::bridge::gateway::ShardManager,
    model::{
        id::{MessageId, UserId},
        user::User,
    },
    prelude::*
};
use typemap::Key;
use crate::menu::HandlerFunc;

// Bot ownerId Container
pub struct BotOwnerContainer;

impl Key for BotOwnerContainer {
    type Value = User;
}

// Shard Manager Container
pub struct ShardManagerContainer;

impl Key for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

// Command Counter
pub struct CommandCounter;

impl Key for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct MessagePaginator;

impl Key for MessagePaginator {
    type Value = HashMap<MessageId, MessagePagination>;
}

pub struct MessagePagination {
    pub author_id: UserId,
    pub current_page: u32,
    pub handler: Option<HandlerFunc>,
    pub message_id: MessageId,
    pub pages: Vec<CreateEmbed>,
}
