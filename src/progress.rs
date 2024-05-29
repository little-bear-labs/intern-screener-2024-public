use indicatif::{ProgressBar, ProgressStyle};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

/// Initializes the progress bar
pub fn initialize_progress_bar() -> Arc<Mutex<ProgressBar>> {
    let pb = Arc::new(Mutex::new(ProgressBar::new_spinner()));
    {
        let pb = pb.lock().unwrap();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner} [{elapsed_precise}] {msg}")
                .expect("Failed to set progress bar template"),
        );
        pb.enable_steady_tick(Duration::from_millis(100));
    }
    pb
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_initialize_progress_bar() {
        let pb = initialize_progress_bar();
        {
            let pb = pb.lock().unwrap();
            assert_eq!(pb.length(), None);
            assert_eq!(pb.position(), 0);
        }
        thread::sleep(Duration::from_secs(1));
        {
            let pb = pb.lock().unwrap();
            pb.finish();
        }
    }
}
