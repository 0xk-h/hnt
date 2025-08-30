use std::{
    io::{self, Write},
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    thread,
    time::Duration,
};

pub struct Loader {
    is_done: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Loader {
    pub fn start() -> Self {
        let is_done = Arc::new(AtomicBool::new(false));
        let flag = is_done.clone();

        let handle = thread::spawn(move || {
            let width = 10;
            let mut pos = 0;
            let mut forward = true;

            while !flag.load(Ordering::Relaxed) {
                let mut bar = vec![' '; width];
                bar[pos] = '=';
                bar[pos+1] = '=';
                print!("\r[{}]", bar.iter().collect::<String>());
                io::stdout().flush().unwrap();

                if forward {
                    pos += 1;
                    if pos == width - 2 {
                        forward = false;
                    }
                } else {
                    pos -= 1;
                    if pos == 0 {
                        forward = true;
                    }
                }
                thread::sleep(Duration::from_millis(80));
            }
            println!("\r[{}] Done!", "=".repeat(width));
        });

        Loader { is_done, handle: Some(handle) }
    }

    pub fn stop(mut self) {
        self.is_done.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
