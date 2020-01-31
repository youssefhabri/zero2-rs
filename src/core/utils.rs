use chrono::prelude::*;
use math::round::floor;
use rand::prelude::*;
use std::ops::Add;
use time::Duration;

use html5ever::rcdom::{Node, NodeData, RcDom};
use html5ever::{parse_document, ParseOpts};
use tendril::TendrilSink;

/// Get the DateTime<Local> for the next Weekday
pub fn next_day(target: Weekday) -> DateTime<Local> {
    let mut dt = Local::now();

    while dt.weekday() != target {
        dt = dt.add(Duration::days(1));
    }

    dt
}

/// Convert a DateTime<Local> to midnight
pub fn to_midnight(datetime: DateTime<Local>) -> DateTime<Local> {
    datetime.date().and_hms(0, 0, 0)
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

/// Generate a random number between the min & max values
pub fn random_num(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min, max)
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
/// Code from the Dissolve crate under the MIT License
///
/// Consumes a string that contains HTML5 tags and outputs a Vec<String>
/// containing the text content inside the tags in a pre-order manner.
///
/// Basic usage:
///
/// ```rust,ignore
/// let input = "<html>Hello World!</html>";
/// let output = strip_html_tags(input);
/// assert_eq!(output, vec!["Hello World!".to_owned()]);
/// ```
pub fn strip_html_tags(input: &str) -> Vec<String> {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .one(input.as_bytes());
    let doc = dom.document;
    get_text(&doc)
}

/// Helper function to return text in text nodes in pre-order traversal.
fn get_text(element: &Node) -> Vec<String> {
    match element.data {
        NodeData::Text { ref contents } => {
            let mut text = vec![(&**contents.borrow()).to_owned()];
            for child in &*element.children.borrow() {
                text.append(&mut get_text(child));
            }
            text
        }
        _ => {
            let mut text = vec![];
            for child in &*element.children.borrow() {
                text.append(&mut get_text(child));
            }
            text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::strip_html_tags;

    #[test]
    fn test_strip_html_tag() {
        let input = "<html>Hello World!</html>";
        let output = strip_html_tags(input);
        assert_eq!(output, vec!["Hello World!".to_owned()]);
    }

    #[test]
    fn test_strip_nested_tags() {
        let input = "<html>Hello<div>World!</div></html>";
        let output = strip_html_tags(input);
        assert_eq!(output, vec!["Hello".to_owned(), "World!".to_owned()]);
    }

    #[test]
    fn test_preorder_traversal() {
        let input = "<html>Hel<div>lo</div>World!</html>";
        let output = strip_html_tags(input);
        assert_eq!(
            output,
            vec!["Hel".to_owned(), "lo".to_owned(), "World!".to_owned()]
        );
    }
}
