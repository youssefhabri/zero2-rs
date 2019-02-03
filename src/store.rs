use std::sync::Arc;
use std::collections::HashMap;

use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::*
};
use typemap::Key;
use serenity::model::user::User;
use serenity::model::id::{UserId, MessageId};
use serenity::builder::CreateEmbed;

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
    pub pages: Vec<CreateEmbed>,
    pub current_page: u32,
    pub message_id: MessageId,
    pub author_id: UserId
}
