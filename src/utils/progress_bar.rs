use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ProgressBar {
    progress: Arc<Mutex<i64>>,
    total: i64,
    bar_width: usize,
    update_interval_ms: u64,
}

impl ProgressBar {
    pub fn new(total: i64) -> Self {
        Self {
            progress: Arc::new(Mutex::new(0)),
            total,
            bar_width: 50,
            update_interval_ms: 100,
        }
    }

    #[allow(dead_code)]
    pub fn with_bar_width(mut self, width: usize) -> Self {
        self.bar_width = width;
        self
    }

    #[allow(dead_code)]
    pub fn with_update_interval(mut self, interval_ms: u64) -> Self {
        self.update_interval_ms = interval_ms;
        self
    }

    pub fn get_progress_counter(&self) -> Arc<Mutex<i64>> {
        Arc::clone(&self.progress)
    }

    pub fn start(self) -> ProgressBarHandle {
        let progress_clone: Arc<Mutex<i64>> = Arc::clone(&self.progress);
        let total = self.total;
        let bar_width = self.bar_width;
        let update_interval = self.update_interval_ms;

        let handle = thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(update_interval));
                let current = *progress_clone.lock().unwrap();
                let percentage = (current as f64 / total as f64) * 100.0;
                let filled_width = (percentage / 100.0 * bar_width as f64) as usize;

                print!("\r\x1B[K"); // Clear the current line
                print!("[");
                for i in 0..bar_width {
                    if i < filled_width {
                        print!("█");
                    } else {
                        print!(" ");
                    }
                }

                print!("] {:.2}% ({}/{} items processed)",
                       percentage, current, total);

                io::stdout().flush().unwrap();

                if current >= total {
                    println!();
                    break;
                }
            }
        });

        ProgressBarHandle { handle }
    }
}

pub struct ProgressBarHandle {
    handle: thread::JoinHandle<()>,
}

impl ProgressBarHandle {
    pub fn wait(self) {
        self.handle.join().unwrap();
    }
}