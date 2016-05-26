extern crate rand;

use rand::Rng;

pub fn random_string() -> String {
    let rstr: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(32)
        .collect();
}
