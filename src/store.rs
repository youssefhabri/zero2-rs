use std::sync::Arc;
use std::collections::HashMap;

use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::*
};
use typemap::Key;
use serenity::model::id::{GuildId, MessageId};
use serenity::builder::CreateEmbed;

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

pub struct GuildPaginator;

impl Key for GuildPaginator {
    type Value = HashMap<GuildId, GuildPagination>;
}

pub struct GuildPagination {
    pub pages: Vec<CreateEmbed>,
    pub current_page: u32,
    pub message_id: MessageId,
}
