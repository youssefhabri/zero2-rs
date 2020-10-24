use serenity::{client::bridge::gateway::ShardManager, prelude::*};
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
