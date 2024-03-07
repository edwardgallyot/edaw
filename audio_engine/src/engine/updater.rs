mod update_manager;

use std::{thread::{JoinHandle, self, Thread}, sync::{atomic::{AtomicBool, Ordering::Relaxed}, Arc}, time};
use anyhow::{Result, anyhow};
use edaw_messaging::{Message, MessageQueue};
use self::update_manager::UpdateManager;

const SLEEP_TIME_MS: u64 = 1;

#[derive(Default)]
pub struct Updater {
    run: Arc<AtomicBool>,
    connection: Option<JoinHandle<()>>,
}

impl Updater {
    pub fn new() -> Updater {
        Updater::default()
    }
   
    pub fn start_updates_thread(&mut self, message_queue: &mut MessageQueue) -> Result<()> {
        let rx = message_queue
            .take_rx()
            .ok_or(anyhow!("no rx"))?;

        let mut manager = UpdateManager::new(rx);

        self.run.store(true, Relaxed);
        let run_clone = self.run.clone();

        let handle = thread::spawn(move || {
            while run_clone.load(Relaxed) {
                Updater::update_state(&mut manager);
                thread::sleep(time::Duration::from_millis(SLEEP_TIME_MS));
            }
        });

        self.connection = Some(handle);
        Ok(())
    }

    pub fn update_state(update_manager: &mut UpdateManager) {
        if let Err(e) = update_manager.handle_updates() {
            eprintln!("error handling updates: {}", e);
        }
    }
}

impl Drop for Updater {
    fn drop(&mut self) {
        self.run.store(false, Relaxed);
        if let Some(c) = self.connection.take() {
            if let Err(e) = c.join() {
                eprintln!("error joining thread: {:?}", e);
            }
        }
    }
}
