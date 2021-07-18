use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::utils::Colour;

use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{ProcessExt, RefreshKind, System, SystemExt};

use crate::core::consts::{BOT_ID, OWNER_ID};
use crate::utils::seconds_to_hrtime;

const BOT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Taken from https://gitlab.com/Mishio595/momiji-rust
/// under the MIT license
#[command]
#[aliases("bi", "binfo", "info")]
#[description = "Show bot information"]
async fn bot_info(context: &Context, message: &Message) -> CommandResult {
    let (guild_count, shard_count, thumbnail) = {
        (
            context.cache.guilds().await.len(),
            context.cache.shard_count().await,
            context.cache.user(BOT_ID).await.unwrap().face(),
        )
    };

    let mut sys = System::new_all();
    sys.refresh_all();

    let mut fields = Vec::new();

    if let Ok(current_pid) = sysinfo::get_current_pid() {
        if let Some(process) = sys.process(current_pid) {
            let uptime = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|current_time| current_time.as_secs() - process.start_time())
                .unwrap_or(0);

            let system_info = format!(
                "Type: KoolOS 4.11 \nUptime: {}",
                seconds_to_hrtime(sys.uptime() as usize)
            );
            fields.push(("System Info", system_info, true));

            let process_info = format!(
                "Memory Usage: {} MB\nCPU Usage {}%\nUptime: {}",
                process.memory() / 1000, // convert to MB
                (process.cpu_usage() * 100.0).round() / 100.0,
                seconds_to_hrtime(uptime as usize)
            );
            fields.push(("Process Info", process_info, true));
        }
    }

    let _ = message.channel_id.send_message(
        &context,
        |m| m.embed(|embed| {
            embed
                .colour(Colour::new(0x005d_a9ff))
                .description("Hi! I'm <@453773001805135883>, a general purpose bot created in [Rust](http://www.rust-lang.org/) using [Serenity](https://github.com/serenity-rs/serenity).")
                .field("Owner", format!("Name: <@{0}>\nID: {0}", OWNER_ID), true)
                .field("Version", format!("v{}", BOT_VERSION), true)
                .field("Counts", format!("Guilds: {}\nShards: {}", guild_count, shard_count), false)
                .fields(fields)
                .thumbnail(thumbnail)
        })
    ).await?;

    Ok(())
}
