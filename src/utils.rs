use rand::distributions::uniform::SampleUniform;
use rand::prelude::Rng;

pub fn random_number<T: SampleUniform>(min: T, max: T) -> T {
    rand::thread_rng().gen_range(min, max)
}
