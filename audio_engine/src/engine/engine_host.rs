use std::{thread::{self, JoinHandle}, sync::{Arc, atomic::{AtomicBool, Ordering::Relaxed}}};

use anyhow::anyhow;

use crate::{audio_channel::{AudioChannel, AudioRx}, AudioEngine};

pub struct AudioEngineHost {
    pub audio_in: AudioChannel,
    pub audio_out: AudioChannel,

    buffer_size: usize,
    num_samples_per_channel: usize,
    num_channels: usize,

    samples: Option<Vec<f32>>,

    run: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

const SLEEP_TIME_NS: u64 = 5000;

impl AudioEngineHost {
    pub fn new(num_samples_per_channel: usize, num_channels: usize) -> anyhow::Result<AudioEngineHost> {
        let buffer_size = num_samples_per_channel * num_channels;
        let mut samples = Vec::new();

        (0..buffer_size).for_each(|_| {
            samples.push(0.0);
        });

        if buffer_size == 0 {
            return Err(anyhow!("no buffer_size"));
        }

        let audio_in = AudioChannel::new(buffer_size);
        let audio_out = AudioChannel::new(buffer_size);
        let samples = Some(samples);
        let run = Arc::new(AtomicBool::default());
        let handle = None;

        let host = AudioEngineHost {
            audio_in,
            audio_out,
            buffer_size,
            num_samples_per_channel,
            num_channels,
            samples,
            run,
            handle,
        };
        Ok(host)
    }

    pub fn collect_samples(samples: &mut Vec<f32>, rx: &mut AudioRx, buffer_size: usize) -> usize {
        let mut received = 0;
        let time = std::time::Instant::now();
        while received <  buffer_size {
            if let Some(audio_in) = rx.consumer.pop() {
                samples[received] = audio_in;
                received += 1; 
            } else {
                if time.elapsed().as_millis() > 40 {
                    break;
                }
                // We yield and do a little sleep to avoid hogging
                // the os thread when waiting for samples from the io.
                thread::yield_now();
                let dur = std::time::Duration::from_nanos(SLEEP_TIME_NS);
                std::thread::sleep(dur);
            }
        }
        received
    }

    pub fn start(&mut self) -> anyhow::Result<()>{
        let run_clone = self.run.clone();
        run_clone.store(true, Relaxed);

        let mut engine = AudioEngine::new(self.num_samples_per_channel, self.num_channels);

        let mut rx = self
            .audio_in
            .take_rx()
            .ok_or(anyhow!("no input rx"))?;

        let mut tx = self
            .audio_out
            .take_tx()
            .ok_or(anyhow!("no output tx"))?;

        let mut samples = self
            .samples
            .take()
            .ok_or(anyhow!("no samples"))?;

        let buffer_size = self.buffer_size;

        let handle = thread::spawn(move || {
            while run_clone.load(Relaxed) {
                let _recv = AudioEngineHost::collect_samples(&mut samples, &mut rx, buffer_size);
                engine.process(samples.as_mut_slice());
                tx.producer.push_slice(samples.as_slice());
            }
        });

        self.handle = Some(handle);
        Ok(())
    }

    pub fn stop(&mut self) {
        println!("Stopping  ");
        self.run.store(false, Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
        println!("Stopped");
    }
}
