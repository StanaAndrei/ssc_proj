use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use crate::core::sha_demo::sha;

const LOW: i64 = -500_000;
const HIGH: i64 = 2_000_000;

pub fn collision_demo() {
    let num_threads = num_cpus::get();
    let hash_set = Arc::new(Mutex::new(HashSet::new()));
    let range_size = (HIGH - LOW + 1) / num_threads as i64;

    let progress = Arc::new(Mutex::new(0i64));
    let total_items = HIGH - LOW + 1;

    let mut handles = vec![];

    let progress_clone = Arc::clone(&progress);
    let progress_handle = thread::spawn(move || {
        let bar_width = 50;

        loop {
            thread::sleep(Duration::from_millis(100));
            let current = *progress_clone.lock().unwrap();
            let percentage = (current as f64 / total_items as f64) * 100.0;

            let filled_width = (percentage / 100.0 * bar_width as f64) as usize;

            print!("\r\x1B[K");// Clear the current line and move cursor to beginning
            print!("["); // Print the progress bar
            for i in 0..bar_width {
                if i < filled_width {
                    print!("█");
                } else {
                    print!(" ");
                }
            }

            print!("] {:.2}% ({}/{} numbers processed)",
                   percentage, current, total_items);

            io::stdout().flush().unwrap();

            if current >= total_items {
                println!();
                break;
            }
        }
    });

    for i in 0..num_threads {
        let thread_hash_set = Arc::clone(&hash_set);
        let thread_progress = Arc::clone(&progress);
        let start = LOW + (i as i64 * range_size);
        let end = if i == num_threads - 1 { HIGH } else { start + range_size - 1 };

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

    progress_handle.join().unwrap();

    let int_sz = HIGH - LOW + 1;
    let len = hash_set.lock().unwrap().len();
    let collisions = int_sz as usize - len;

    if collisions == 0 {
        println!("No collision detected");
    } else {
        println!("{} collisions detected", collisions);
    }
}