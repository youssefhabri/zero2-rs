use async_trait::async_trait;
use serenity::model::prelude::{MessageId, Reaction, UserId};
use serenity::{client::bridge::gateway::ShardManager, prelude::*};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct PaginationContainer;

impl TypeMapKey for PaginationContainer {
    type Value = Arc<Mutex<HashMap<MessageId, PaginationInfo>>>;
}

#[async_trait]
pub trait Pagination: Send + Sync {
    async fn handle(&mut self, _: &Context, _: &Reaction);
    fn len(&self) -> usize;
    fn cursor(&self) -> usize;
    fn set_cursor(&mut self, _: usize);
}

// TODO Think of a better name
pub struct PaginationInfo {
    pub author: UserId,
    pub ended: bool,
    pub pagination: Box<dyn Pagination>,
}

impl PaginationInfo {
    pub fn new(author: UserId, pagination: Box<dyn Pagination>) -> PaginationInfo {
        let ended = false;
        PaginationInfo {
            author,
            ended,
            pagination,
        }
    }
}
