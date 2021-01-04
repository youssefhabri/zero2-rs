use rand::distributions::WeightedIndex;
use rand::prelude::{thread_rng, Distribution, Rng};

/// Generate a random number between the min & max values
pub fn random_number(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min, max)
}

pub fn random_with_weights<T: Clone>(choices: &[T], weights: &[u32]) -> Result<T, String> {
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
