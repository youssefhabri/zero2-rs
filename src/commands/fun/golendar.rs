use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::{Args, Command, CommandError};

use chrono::{DateTime, Date, Utc};
use time::Duration;
use crate::utils::format_time_long;

const START_DATE: &str = "2019-02-27T00:00:00+00:00";

pub struct GolendarCommand;

impl Command for GolendarCommand {
    fn execute(&self, _: &mut Context, message: &Message, _: Args) -> Result<(), CommandError> {

        let start = DateTime::parse_from_rfc3339(START_DATE)?;
        let now = Utc::now();
        let difference = now.timestamp_millis() - start.timestamp_millis();

        let duration = Duration::milliseconds(difference);

        let _ = message.channel_id.say(
            format!("{} since the start of the Golumpian Calendar.", format_time_long(duration.num_minutes() as f64))
        );

        Ok(())
    }
}
