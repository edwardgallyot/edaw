use std::{sync::Arc, mem::MaybeUninit};

use ringbuf::{Consumer, SharedRb};

type HeapRbRx = Consumer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>;

pub struct AudioRx {
    pub consumer:  HeapRbRx,
}

