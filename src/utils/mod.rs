pub mod extension;
pub mod db_helper;

use rand::{thread_rng, Rng, distributions::Alphanumeric};

pub fn get_random_string(len: usize) -> String {
    String::from_utf8(thread_rng().sample_iter(&Alphanumeric).take(len).collect()).unwrap()
}

pub fn string_to_i64(str: &String) -> i64 {
    str.parse().unwrap_or(0)
}