use chrono::prelude::*;
use math::round::floor;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::ops::Add;
use time::Duration;

use serenity::model::prelude::Message;
use serenity::prelude::Context;

use super::store::{Command, CommandLogger};

#[inline]
pub fn log_command(ctx: &mut Context, msg: &Message, cmd: &str) {
    let mut data = ctx.data.write();
    let cmd_logger = data.get_mut::<CommandLogger>().unwrap();
    cmd_logger.insert(
        msg.id,
        Command {
            name: cmd.to_string(),
            message: msg.content.clone(),
            user_id: msg.author.id,
            time: msg.timestamp,
        },
    );
}

/// Get the DateTime<Local> for the next Weekday
pub fn next_day(target: Weekday) -> DateTime<Local> {
    let mut dt = Local::now();

    while dt.weekday() != target {
        dt = dt.add(Duration::days(1));
    }

    dt
}

/// Convert a Weekday enum to weekday name
pub fn weekday_to_string(weekday: Weekday) -> String {
    let weekday: &str = match weekday {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };

    weekday.to_owned()
}

/// Convert month (number) to short format name
pub fn month_to_string<'a>(month: u32) -> &'a str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => unreachable!(),
    }
}

/// Returns formated time from minutes
pub fn format_time(time_minutes: f64) -> String {
    let minutes = floor(time_minutes % 60.0, 0);
    let hours = floor((time_minutes / 60.0) % 24.0, 0);
    let days = floor(time_minutes / 60.0 / 24.0, 0);

    if days > 0.0 {
        return format!("{} days, {}:{}", days, hours, minutes);
    } else if hours > 0.0 {
        return format!("{} hours, {} minutes", hours, minutes);
    }

    format!("{} minutes", minutes)
}

pub fn _format_time_long(time_minutes: f64) -> String {
    let minutes = floor(time_minutes % 60.0, 0);
    let hours = floor((time_minutes / 60.0) % 24.0, 0);
    let days = floor(time_minutes / 60.0 / 24.0, 0);

    if days > 0.0 {
        return format!("{} days, {} hours {} minutes", days, hours, minutes);
    } else if hours > 0.0 {
        return format!("{} hours, {} minutes", hours, minutes);
    }

    format!("{} minutes", minutes)
}

/// Generate a random number between the min & max values
pub fn random_num(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min, max)
}

pub fn random_with_weights<T: Clone>(choices: &Vec<T>, weights: &Vec<u32>) -> Result<T, String> {
    if choices.len() != weights.len() {
        return Err("choices and weights need to be the same size.".to_string());
    }

    let mut rng = thread_rng();

    let dist = WeightedIndex::new(weights).unwrap();
    let result = dist.sample(&mut rng);

    Ok(choices[result].clone())
}

/// Converts a time in seconds to a human readable string
/// Taken from https://gitlab.com/Mishio595/momiji-rust
/// under the MIT license
const WEEK: usize = 60 * 60 * 24 * 7;
const DAY: usize = 60 * 60 * 24;
const HOUR: usize = 60 * 60;
const MIN: usize = 60;

pub fn seconds_to_hrtime(secs: usize) -> String {
    let word = ["week", "day", "hour", "min", "sec"];
    fn make_parts(t: usize, steps: &[usize], mut accum: Vec<usize>) -> Vec<usize> {
        match steps.split_first() {
            None => accum,
            Some((s, steps)) => {
                accum.push(t / *s);
                make_parts(t % *s, steps, accum)
            }
        }
    }

    make_parts(secs, &[WEEK, DAY, HOUR, MIN, 1], Vec::new())
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if s > &0 {
                if s > &1 {
                    Some(format!("{} {}s", s, word[i]))
                } else {
                    Some(format!("{} {}", s, word[i]))
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}
