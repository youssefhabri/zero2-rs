use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{Args, CommandResult, macros::command},
    model::channel::Message,
    prelude::*,
};
use crate::core::store::ShardManagerContainer;


#[command("ping")]
fn ping_command(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let data = context.data.read();

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = message.reply(context, "There was a problem getting the shard manager");

            return Ok(());
        },
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = match runners.get(&ShardId(context.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = message.reply(context, "No shard found");

            return Ok(());
        },
    };

    let _ = message.reply(
        context,
        &format!("The shard latency is {}ms", runner.latency.unwrap().as_millis())
    );

    Ok(())
}

