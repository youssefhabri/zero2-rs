use rand::distributions::uniform::SampleUniform;
use rand::prelude::Rng;

pub fn random_number<T: SampleUniform>(min: T, max: T) -> T {
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
