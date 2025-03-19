use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::core::sha_demo::sha;
use crate::utils::progress_bar::ProgressBar;


pub fn collision_demo_rng(low: i64, high: i64) {
    println!("Collision demo [{} {}]", low, high);
    let num_threads = num_cpus::get();
    let hash_set: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
    let range_size = (high - low + 1) / num_threads as i64;
    let total_items = high - low + 1;

    let progress_bar = ProgressBar::new(total_items);
    let progress: Arc<Mutex<i64>> = progress_bar.get_progress_counter();
    let progress_handle = progress_bar.start();

    let mut handles = vec![];

    for i in 0..num_threads {
        let thread_hash_set: Arc<Mutex<HashSet<String>>> = Arc::clone(&hash_set);
        let thread_progress: Arc<Mutex<i64>> = Arc::clone(&progress);
        let start = low + (i as i64 * range_size);
        let end = if i == num_threads - 1 { high } else { start + range_size - 1 };

        let handle = thread::spawn(move || {
            let mut local_set = HashSet::new();
            let mut local_count = 0;

            for nr in start..=end {
                let hex_sha = sha::get_str_hash(&nr.to_string());
                local_set.insert(hex_sha);

                local_count += 1;
                if local_count % 10000 == 0 {  // Update progress periodically
                    let mut progress = thread_progress.lock().unwrap();
                    *progress += 10000;
                }
            }

            // Update final progress
            let mut progress = thread_progress.lock().unwrap();
            *progress += local_count % 10000;

            let mut global_set = thread_hash_set.lock().unwrap();
            global_set.extend(local_set);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    progress_handle.wait();

    let int_sz = high - low + 1;
    let len = hash_set.lock().unwrap().len();
    let collisions = int_sz as usize - len;

    if collisions == 0 {
        println!("No collision detected(as expected)");
    } else {
        println!("{} collisions detected", collisions);
    }
}