use std::collections::{HashSet};
use crate::core::sha_demo::sha;
use crate::utils::progress_bar::ProgressBar;
use crate::utils::str_rand::generate_random_string;

const NUM_STRINGS: usize = 1_000_000;
const MIN_LENGTH: usize = 5;
const MAX_LENGTH: usize = 50;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";

pub fn collision_demo_str() {
    let mut gen_set: HashSet<String> = HashSet::new();
    let mut sha_set: HashSet<String> = HashSet::new();
    let progress_bar = ProgressBar::new(NUM_STRINGS as i64);
    let progress = progress_bar.get_progress_counter();
    let progress_handle = progress_bar.start();

    let mut strings_generated = 0;

    while strings_generated < NUM_STRINGS {
        let s = generate_random_string(MIN_LENGTH, MAX_LENGTH, CHARSET);

        if !gen_set.contains(&s) {
            gen_set.insert(s.clone());
            let s_hash = sha::get_str_hash(&s);
            sha_set.insert(s_hash);

            strings_generated += 1;

            // Update progress
            let mut progress_counter = progress.lock().unwrap();
            *progress_counter += 1;
        }
    }

    progress_handle.wait();

    let collisions = NUM_STRINGS - sha_set.len();
    if collisions == 0 {
        println!("No collision detected");
    } else {
        println!("{} collisions detected", collisions);
    }
}