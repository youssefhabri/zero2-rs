use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, FixedOffset};
use serenity::{
    builder::CreateEmbed,
    client::bridge::gateway::ShardManager,
    model::{
        id::{MessageId, UserId},
        user::User,
    },
    prelude::*,
};

use crate::menu::HandlerFunc;

// Bot ownerId Container
pub struct BotOwnerContainer;

impl TypeMapKey for BotOwnerContainer {
    type Value = User;
}

// Shard Manager Container
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

// Command Counter
pub struct CommandLogger;

impl TypeMapKey for CommandLogger {
    type Value = HashMap<MessageId, Command>;
}

#[derive(Clone, Debug)]
pub struct Command {
    pub message: String,
    pub name: String,
    pub user_id: UserId,
    pub time: DateTime<FixedOffset>,
}

// Message Pagination
pub struct MessagePaginator;

impl TypeMapKey for MessagePaginator {
    type Value = HashMap<MessageId, MessagePagination>;
}

pub struct MessagePagination {
    pub author_id: UserId,
    pub current_page: u32,
    pub handler: Option<HandlerFunc>,
    pub message_id: MessageId,
    pub pages: Vec<CreateEmbed>,
    pub deleted: bool,
}

pub enum RichInteractionKind {
    Pagination,
}

pub struct RichInteraction {
    kind: RichInteractionKind,
}
