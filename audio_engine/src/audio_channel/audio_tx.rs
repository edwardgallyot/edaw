use std::{sync::Arc, mem::MaybeUninit};

use ringbuf::{Producer, SharedRb, HeapRb};

type HeapRbTx = Producer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>;

pub struct AudioTx {
    pub producer:  HeapRbTx,
}
