use ::anilist::client::Error as AniListError;
use async_trait::async_trait;
use serenity::framework::standard::CommandError;
use serenity::model::prelude::{MessageId, Reaction, UserId};
use serenity::prelude::{Context, RwLock, TypeMapKey};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

pub struct PaginationContainer;

impl TypeMapKey for PaginationContainer {
    type Value = Arc<RwLock<HashMap<MessageId, PaginationInfo>>>;
}

#[derive(Debug, Error)]
pub enum PaginationError {
    #[error("Serenity command error: {0}")]
    SerenityCommand(#[from] CommandError),

    #[error("AniList client error: {0}")]
    AniListClient(#[from] AniListError),
}

pub type PaginationResult = Result<(), PaginationError>;

#[async_trait]
pub trait Pagination: Send + Sync {
    fn name(&self) -> String;
    fn count(&self) -> usize;
    fn cursor(&self) -> usize;
    fn set_cursor(&mut self, _: usize);
    async fn handle(&mut self, _: &Context, _: &Reaction) -> PaginationResult;
}

// TODO Think of a better name
pub struct PaginationInfo {
    pub author: UserId,
    pub ended: bool,
    pub pagination: Box<dyn Pagination>,
}

impl PaginationInfo {
    pub fn new<P>(author: UserId, pagination: P) -> PaginationInfo
    where
        P: Pagination + 'static,
    {
        let ended = false;
        PaginationInfo {
            author,
            ended,
            pagination: Box::new(pagination),
        }
    }
}
