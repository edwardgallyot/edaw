mod state;
mod connection;
mod engine_host;
mod updater;

use edaw_messaging::MessageQueue;
pub use engine_host::*;
use state::State;
use connection::Connection;
use updater::Updater;

pub struct AudioEngine {
    num_samples_per_channel: usize,
    num_channels: usize,
    state: State,
    connection: Connection,
    updater: Updater,
}

impl AudioEngine {
    pub fn new(num_samples_per_channel: usize, num_channels: usize) -> Self {
        let state = State::new();
        let connection = Connection::new();
        let updater = Updater::new();

        Self {
            num_samples_per_channel,
            num_channels,
            state,
            connection,
            updater,
        }
    }

    fn prepare(&mut self) -> anyhow::Result<()> {
        let mut message_queue = MessageQueue::new();
        self.connection.start_connection_thread(&mut message_queue)?;
        self.updater.start_updates_thread(&mut message_queue)?;
        Ok(())
    }

    fn make_dual_mono(&mut self, data: &mut [f32]) {
        data.chunks_exact_mut(2).for_each(|samples|{
            if let [left, right] = samples {
                *right = *left;
            } else {
                eprintln!("invalid block size");
            }
            // No else here, the buffer should be a power of 2 and we don't
            // bail out of process functions.
        });
    }

    fn apply_hard_clip(&mut self, data: &mut [f32], hard_clip_threshold: f32) {
        data.iter_mut().for_each(|s|{
            if *s > hard_clip_threshold {
                *s = hard_clip_threshold;
            } else if *s < -hard_clip_threshold {
                *s = -hard_clip_threshold;
            }
        });
    }

    pub fn process(&mut self, data: &mut [f32]) {
        data.iter_mut().for_each(|s| {
            *s *= 1.0;
        });
        self.make_dual_mono(data);

        let hard_clip_threshold = 1.0;
        let hard_clip_threshold = 0.01;
        self.apply_hard_clip(data, hard_clip_threshold);
    }
}
