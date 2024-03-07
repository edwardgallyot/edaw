use std::{sync::Arc, mem::MaybeUninit};

use ringbuf::{Producer, SharedRb};

type HeapRbTx = Producer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>;

pub struct AudioTx {
    producer:  HeapRbTx,
}

impl AudioTx {
    pub fn new(producer: HeapRbTx) -> AudioTx {
        AudioTx {
            producer
        }
    }

    pub fn push_slice(&mut self, samples: &mut [f32]) -> usize  {
        self.producer.push_slice(samples)
    }
}

