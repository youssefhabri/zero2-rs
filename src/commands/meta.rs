use serenity::{
    prelude::*,
    model::channel::Message,
    client::bridge::gateway::ShardId,
    framework::standard::{Args, Command, CommandError},
};
use crate::store::ShardManagerContainer;


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

        let _ = message.reply(&format!("The shard latency is {}", runner.latency.unwrap().as_millis()));

        Ok(())
    }
}

use std::str;
use std::thread;
use std::time::Duration;

#[derive(RustEmbed)]
#[folder = "assets/ascii"]
struct Asset;


fn load_frame(frame: u32) -> String {
    let file = format!("frame_{}.txt", frame);
    let asset = match Asset::get(&file) {
        Some(asset) => asset,
        None => panic!("Error loading: {}", file)
    };
    str::from_utf8(&asset).expect(file.as_str()).to_owned()
}

pub struct Anim;

impl Command for Anim {
    fn execute(&self, _ctx: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {

        let params = if args.full().len() > 0 {
            args.multiple::<String>().unwrap()
        } else { vec![] };

        let repeat = if params.len() > 0 { params[0].clone().parse::<u32>().unwrap_or(1) } else { 1 };

        let init_frame = load_frame(0);

        let sending = message.channel_id.send_message(|m| m.content(format!("```{}```", init_frame)));

        if let Ok(msg) = sending {
            let end = 24;
            for i in 1..(end * repeat) {
                let frame = load_frame(i % end);
                let _ = msg.channel_id.edit_message(msg.id, |m| m.content(format!("```{} ```", frame)));
                thread::sleep(Duration::from_millis(900));
            }

            let _ = msg.delete();
        }

        Ok(())
    }
}