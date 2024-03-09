use std::{
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc,
    },
    thread::{self, JoinHandle},
};

use anyhow::anyhow;

use crate::{audio_channel::AudioChannel, AudioEngine};

pub struct AudioEngineHost {
    pub audio_in: AudioChannel,
    pub audio_out: AudioChannel,

    buffer_size: usize,
    num_samples_per_channel: usize,
    num_channels: usize,

    run: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl AudioEngineHost {
    pub fn new(
        num_samples_per_channel: usize,
        num_channels: usize,
    ) -> anyhow::Result<AudioEngineHost> {
        let buffer_size = num_samples_per_channel * num_channels;

        if buffer_size == 0 {
            return Err(anyhow!("no buffer_size"));
        }

        let audio_in = AudioChannel::new(buffer_size);
        let audio_out = AudioChannel::new(buffer_size);
        let run = Arc::new(AtomicBool::default());
        let handle = None;

        let host = AudioEngineHost {
            audio_in,
            audio_out,
            buffer_size,
            num_samples_per_channel,
            num_channels,
            run,
            handle,
        };
        Ok(host)
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let run_clone = self.run.clone();
        run_clone.store(true, Relaxed);

        let mut engine = AudioEngine::new(self.num_samples_per_channel, self.num_channels);

        engine.prepare()?;

        let mut rx = self.audio_in.take_rx().ok_or(anyhow!("no input rx"))?;

        let mut tx = self.audio_out.take_tx().ok_or(anyhow!("no output tx"))?;

        let mut samples = Vec::new();

        (0..self.buffer_size).for_each(|_| {
            samples.push(0.0);
        });

        let handle = thread::spawn(move || {
            while run_clone.load(Relaxed) {
                rx.collect_samples(samples.as_mut_slice());
                engine.process(samples.as_mut_slice());
                tx.push_slice(samples.as_mut_slice());
            }
        });

        self.handle = Some(handle);
        Ok(())
    }
}

impl Drop for AudioEngineHost {
    fn drop(&mut self) {
        self.run.store(false, Relaxed);
        if let Some(h) = self.handle.take() {
            if let Err(e) = h.join() {
                eprintln!("error joining thread: {:?}", e);
            }
        }
    }
}
