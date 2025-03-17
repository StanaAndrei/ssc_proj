use rand::rng;
use rand::Rng;
use std::collections::{HashSet};
//use std::sync::{Arc, Mutex};
//use std::thread;
//use std::time::Duration;
//use std::io::{self, Write};
use crate::core::sha_demo::sha;

const NUM_STRINGS: usize = 1_000;
const MIN_LENGTH: usize = 5;
const MAX_LENGTH: usize = 50;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";

fn generate_random_string(min_length: usize, max_length: usize) -> String {
    let mut rngo = rng();
    let length = rngo.random_range(min_length..=max_length);

    (0..length)
        .map(|_| {
            let idx = rngo.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn collision_demo_str() {
    let mut gen_set: HashSet<String> = HashSet::new();
    let mut sha_set: HashSet<String> = HashSet::new();

    for _i in 0..NUM_STRINGS {
        let mut s: String;
        loop {
            s = generate_random_string(MIN_LENGTH, MAX_LENGTH);
            if !gen_set.contains(&s) {
                gen_set.insert(s.clone());
                break;
            }
        }
        let s_hash = sha::get_str_hash(&s);
        sha_set.insert(s_hash);
    }

    let collisions = NUM_STRINGS - sha_set.len();
    if collisions == 0 {
        println!("No collision detected");
    } else {
        println!("{} collisions detected", collisions);
    }
}