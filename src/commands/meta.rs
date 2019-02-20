use std::sync::Arc;
use sysinfo::{
    ProcessExt,
    SystemExt,
    System,
    get_current_pid
};
use serenity::{
    CACHE,
    client::bridge::gateway::ShardId,
    framework::standard::{Args, Command, CommandError, CommandOptions},
    model::channel::Message,
    prelude::*,
    utils::Colour
};
use crate::utils::seconds_to_hrtime;
use crate::store::{BotOwnerContainer, ShardManagerContainer};


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

        let _ = message.reply(&format!("The shard latency is {}ms", runner.latency.unwrap().as_millis()));

        Ok(())
    }
}

/// Taken from https://gitlab.com/Mishio595/momiji-rust
/// under the MIT license
pub struct BotInfo;
impl Command for BotInfo {
    fn options(&self) -> Arc<CommandOptions> {
        let default = CommandOptions::default();
        let options = CommandOptions {
            desc: Some("Information about the bot.".to_string()),
            usage: Some("".to_string()),
            aliases: vec!["bi", "binfo"].iter().map(|e| e.to_string()).collect(),
            ..default
        };
        Arc::new(options)
    }

    fn execute(&self, ctx: &mut Context, message: &Message, _: Args) -> Result<(), CommandError> {
        use serenity::builder::CreateEmbed;

        let data = ctx.data.lock();
        let (guild_count, shard_count, thumbnail) = {
            let cache = CACHE.read();
            (cache.guilds.len(), cache.shard_count, cache.user.face())
        };
        let owner = data.get::<BotOwnerContainer>().expect("Failed to get owner");
        let sys = System::new();
        let embed = CreateEmbed::default()
                .description("Hi! I'm <@453773001805135883>, a general purpose bot created in [Rust](http://www.rust-lang.org/) using [Serenity](https://github.com/serenity-rs/serenity).")
                .field("Owner", format!(
                    "Name: {}\nID: {}"
                    ,owner.tag()
                    ,owner.id)
                    ,true)
                .field("Counts", format!(
                    "Guilds: {}\nShards: {}"
                    ,guild_count
                    ,shard_count)
                    ,false)
                .thumbnail(thumbnail)
                .colour(Colour::new(0x5da9ff));
        if let Some(process) = sys.get_process(get_current_pid()) {
            message.channel_id.send_message(|m| m
                .embed(|_| embed
                    .field("System Info", format!(
                        "Type: {} {}\nUptime: {}"
                        ,sys_info::os_type().unwrap_or(String::from("OS Not Found"))
                        ,sys_info::os_release().unwrap_or(String::from("Release Not Found"))
                        ,seconds_to_hrtime(sys.get_uptime() as usize))
                        ,true)
                    .field("Process Info", format!(
                        "Memory Usage: {} MB\nCPU Usage {}%\nUptime: {}"
                        ,process.memory()/1000 // convert to MB
                        ,(process.cpu_usage()*100.0).round()/100.0 // round to 2 decimals
                        ,seconds_to_hrtime((sys.get_uptime() - process.start_time()) as usize))
                        ,true)
            ))?;

        } else {
            message.channel_id.send_message(|m| m
                .embed(|_| embed
            ))?;
        }
        Ok(())
    }
}
