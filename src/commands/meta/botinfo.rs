use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
    utils::Colour,
};
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

use crate::core::store::BotOwnerContainer;
use crate::core::utils::seconds_to_hrtime;

const BOT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Taken from https://gitlab.com/Mishio595/momiji-rust
/// under the MIT license
#[command]
#[aliases("bi", "binfo", "info")]
#[description = "Show bot information"]
fn bot_info(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let data = context.data.read();
    let (guild_count, shard_count, thumbnail) = {
        let cache = context.cache.read();
        (cache.guilds.len(), cache.shard_count, cache.user.face())
    };
    let owner = data
        .get::<BotOwnerContainer>()
        .expect("Failed to get owner");
    let sys = System::new();

    message.channel_id.send_message(
        &context.http,
        |m| m.embed(|embed| {
            embed
                .colour(Colour::new(0x005d_a9ff))
                .description("Hi! I'm <@453773001805135883>, a general purpose bot created in [Rust](http://www.rust-lang.org/) using [Serenity](https://github.com/serenity-rs/serenity).")
                .field("Owner", format!(
                    "Name: {}\nID: {}"
                    ,owner.tag()
                    ,owner.id)
                       ,true)
                .field("Version", format!("v{}", BOT_VERSION), true)
                .field("Counts", format!(
                    "Guilds: {}\nShards: {}"
                    ,guild_count
                    ,shard_count)
                       ,false)
                .thumbnail(thumbnail);

            if let Ok(current_pid) = get_current_pid() {
                if let Some(process) = sys.get_process(current_pid) {
                    let uptime = if let Ok(current_time) = SystemTime::now().duration_since(UNIX_EPOCH) {
                        current_time.as_secs() - process.start_time()
                    } else { 0 } as usize;

                    embed
                        .field("System Info", format!(
                            "Type: {} {}\nUptime: {}"
                            ,sys_info::os_type().unwrap_or_else(|_| String::from("OS unknown"))
                            ,if sys_info::os_type().unwrap() == "Linux" {
                                String::from("2.4")
                            } else {
                                sys_info::os_release().unwrap_or_else(|_| String::from("(release unknown)"))
                            }
                            ,seconds_to_hrtime(sys.get_uptime() as usize))
                            ,true)
                        .field("Process Info", format!(
                            "Memory Usage: {} MB\nCPU Usage {}%\nUptime: {}"
                            ,process.memory()/1000 // convert to MB
                            ,(process.cpu_usage()*100.0).round()/100.0 // round to 2 decimals
                            ,seconds_to_hrtime(uptime))
                            ,true);
                }
            }

            embed
        })
    )?;

    Ok(())
}
