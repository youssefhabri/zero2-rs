use chrono::prelude::*;
use time::Duration;
use std::ops::Add;
use rand::prelude::*;

pub fn next_day(target: Weekday) -> DateTime<Local> {
    let mut dt = Local::now();

    while dt.weekday() != target {
        dt = dt.add(Duration::days(1));
    }

    dt
}

pub fn to_midnight(datetime: DateTime<Local>) -> DateTime<FixedOffset> {
    let midnight = format!(
        "{}-{:02}-{}T00:00:00{}",
        datetime.date().year(), datetime.date().month(), datetime.date().day(), "+01:00"
    );

    DateTime::parse_from_rfc3339(midnight.as_str()).unwrap()
}

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

pub fn random_num(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min, max)
}
