extern crate rand;

use rand::Rng;

pub fn random_string(count: usize) -> String {
    let rstr: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(count)
        .collect();
    return rstr;
}

pub fn read_bytes(str: &String, offset: u64, size: u32) -> &[u8] {
    let bytes = str.as_bytes();
    let start = offset as usize;
    let stop = start + size as usize;
    let end = if bytes.len() < stop { bytes.len() } else { stop };
    &bytes[start..end]
}
