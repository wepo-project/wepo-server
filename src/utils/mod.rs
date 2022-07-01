pub mod extension;

use rand::{thread_rng, Rng, distributions::Alphanumeric};

pub fn get_random_string(len: usize) -> String {
    String::from_utf8(thread_rng().sample_iter(&Alphanumeric).take(len).collect()).unwrap()
}