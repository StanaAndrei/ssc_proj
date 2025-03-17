use rand::{rng, Rng};

pub fn generate_random_string(min_length: usize, max_length: usize, chars: &[u8]) -> String {
    let mut rngo = rng();
    let length = rngo.random_range(min_length..=max_length);

    (0..length)
        .map(|_| {
            let idx = rngo.random_range(0..chars.len());
            chars[idx] as char
        })
        .collect()
}