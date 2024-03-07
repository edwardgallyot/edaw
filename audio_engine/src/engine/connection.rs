mod connection_manager;

use std::{thread::{JoinHandle, self}, sync::{atomic::{AtomicBool, Ordering::Relaxed}, Arc}};

use anyhow::{Result, anyhow};
use connection_manager::ConnectionManager;
use edaw_messaging::MessageQueue;

#[derive(Default)]
pub struct Connection {
    run: Arc<AtomicBool>,
    connection: Option<JoinHandle<()>>,
}

impl Connection {
    pub fn new() -> Connection {
        Connection::default()
    }

    pub fn start_connection_thread(&mut self, message_queue: &mut MessageQueue) -> Result<()> {
        let tx = message_queue
            .take_tx()
            .ok_or(anyhow!("no tx"))?;

        let mut manager = ConnectionManager::new(tx)?;

        self.run.store(true, Relaxed);
        let run_clone = self.run.clone();

        let handle = thread::spawn(move || {
            while run_clone.load(Relaxed) {
                Connection::accept_connection(&mut manager);
            }
        });

        self.connection = Some(handle);
        Ok(())
    }

    fn accept_connection(manager: &mut ConnectionManager) {
        if let Err(e) = manager.accept_connections() {
            eprintln!("error accepting connections to audio engine {}", e);
        }
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

