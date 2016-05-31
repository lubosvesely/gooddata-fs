extern crate rand;

use rand::Rng;

pub fn random_string(count: usize) -> String {
    let rstr: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(count)
        .collect();
    return rstr;
}
