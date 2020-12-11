use crate::core::store::ShardManagerContainer;
use serenity::client::bridge::gateway::ShardId;
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::time::Duration;

#[command]
async fn ping(context: &Context, message: &Message, _: Args) -> CommandResult {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let shard_manager = {
        let data = context.data.read().await;
        let shard_manager = data
            .get::<ShardManagerContainer>()
            .ok_or_else(|| CommandError::from("Failed to get ShardManager"))?;

        shard_manager.clone()
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = runners
        .get(&ShardId(context.shard_id))
        .ok_or_else(|| CommandError::from("No shard found"))?;

    let latency = runner
        .latency
        .unwrap_or_else(|| Duration::from_millis(0))
        .as_millis();

    let _ = message
        .reply(&context, &format!("The shard latency is {}ms", latency))
        .await?;

    Ok(())
}
