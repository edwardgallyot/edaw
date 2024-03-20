use std::{mem::MaybeUninit, sync::Arc, thread};

use ringbuf::{Consumer, SharedRb};

type HeapRbRx = Consumer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>;

const SLEEP_TIME_NS: u64 = 5000;

pub struct AudioRx {
    consumer: HeapRbRx,
}

impl AudioRx {
    pub fn new(consumer: HeapRbRx) -> AudioRx {
        AudioRx { consumer }
    }

    pub fn collect_samples(&mut self, samples: &mut [f32]) {
        let mut received = 0;
        let time = std::time::Instant::now();
        while received < samples.len() {
            if let Some(audio_in) = self.consumer.pop() {
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
    }
}
