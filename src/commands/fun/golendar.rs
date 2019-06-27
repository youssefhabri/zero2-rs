use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::core::utils::{month_to_string, weekday_to_string};
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};

const TIMESTAMP_START: i64 = 1_551_312_000_000; //1551308400000;

#[command]
fn golendar(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let now = Utc::now();
    let difference = now.timestamp_millis() - TIMESTAMP_START;

    let date = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(difference / 1000, (difference % 1000) as u32),
        Utc,
    );

    // TODO Add a `since` command?
    // let duration = Duration::milliseconds(difference);

    // let _ = message.channel_id.say(
    //     format!("{} since the start of the Golumpian Calendar.", format_time_long(duration.num_minutes() as f64))
    // );

    let _ = message.channel_id.say(
        &context.http,
        format!(
            "{} {}, {} {:0>4} - {:0>2}:{:0>2}:{:0>2} UTC",
            weekday_to_string(date.weekday()),
            date.day(),
            month_to_string(date.month()),
            date.year() - 1970,
            date.hour(),
            date.minute(),
            date.second()
        ),
    );

    Ok(())
}
