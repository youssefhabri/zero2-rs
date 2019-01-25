use serenity::{
    prelude::*,
    model::channel::Message,
    client::bridge::gateway::ShardId,
    framework::standard::{Args, Command, CommandError},
};
use crate::store::ShardManagerContainer;


pub struct Test;

impl Command for Test {
    fn execute(&self, ctx: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {


        Ok(())
    }
}

pub struct Ping;

impl Command for Ping {
    fn execute(&self, context: &mut Context, message: &Message, _: Args) -> Result<(), CommandError> {
        // The shard manager is an interface for mutating, stopping, restarting, and
        // retrieving information about shards.
        let data = context.data.lock();

        let shard_manager = match data.get::<ShardManagerContainer>() {
            Some(v) => v,
            None => {
                let _ = message.reply("There was a problem getting the shard manager");

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
                let _ = message.reply("No shard found");

                return Ok(());
            },
        };

        let _ = message.reply(&format!("The shard latency is {:?}", runner.latency));

        Ok(())
    }
}