use std::{thread::{JoinHandle, self}, sync::{atomic::{AtomicBool, Ordering::Relaxed}, Arc}, time::Duration};

#[derive(Default)]
pub struct Connection {
    run: Arc<AtomicBool>,
    connection: Option<JoinHandle<()>>,
}

impl Connection {
    pub fn new() -> Connection {
        Connection::default()
    }

    pub fn start_connection_thread(&mut self) {
        self.run.store(true, Relaxed);
        let run_clone = self.run.clone();
        let handle = thread::spawn(move || {
            while run_clone.load(Relaxed) {
                // TODO: manage the connections here.
                thread::sleep(Duration::from_secs_f64(1.0));
            }
        });
        self.connection = Some(handle);
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.run.store(false, Relaxed);
        if let Some(c) = self.connection.take() {
            if let Err(e) = c.join() {
                eprintln!("error joining thread: {:?}", e);
            }
        }
    }
}

