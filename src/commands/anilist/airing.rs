use chrono::{Local, Weekday};
use std::ops::Add;
use time::Duration;

use serenity::framework::standard::{
    Args,
    CommandResult,
    macros::command
};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::models::anilist::airing_schedule::AiringSchedule;
use crate::core::utils::{next_day, to_midnight, weekday_to_string};


#[command("airing")]
#[aliases("airs")]
#[usage = "[weekday]"]
#[description = "Show airing anime for a given/current day"]
fn airing_command(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let keyword = args.parse::<String>().unwrap_or_else(|_| "".to_string());

    let (start, day) = if keyword.is_empty() {
        (to_midnight(Local::now()), "Today".to_owned())
    } else {
        let day = keyword;
        match day.parse::<Weekday>() {
            Ok(day) => {
                (to_midnight(next_day(day)), weekday_to_string(day))
            }
            Err(_) => (to_midnight(Local::now()), "Today".to_owned())
        }
    };

    let results: Vec<AiringSchedule> = client::search_airing_schedule(
        start.timestamp(),
        start.add(Duration::days(1)).timestamp()
    );

    if !results.is_empty() {
        let mut airing = vec![];

        for item in results {
            airing.push(item.to_url());
        }

        let _ = message.channel_id.send_message(
            &context.http,
            |m| m
                .embed(|e| e
                    .color(3447003)
                    .title(format!("Airing Schedule for {}", day))
                    .description(airing.join("\n"))
                    .footer(|f| f
                        .icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                        .text("Powered by AniList"))
                )
        );
    } else {
        let _ = message.channel_id.say(&context.http, "Error checking the airing schedule".to_string());
    }

    Ok(())
}