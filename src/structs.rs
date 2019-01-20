use std::sync::Arc;
use std::collections::HashMap;

use serenity::{
    client::{
        EventHandler,
        bridge::gateway::ShardManager
    },
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*
};
use typemap::Key;

pub struct ShardManagerContainer;

impl Key for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct CommandCounter;

impl Key for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}