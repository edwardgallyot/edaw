use std::{sync::Arc, mem::MaybeUninit};

use ringbuf::{Consumer, ring_buffer::{RbRef, Container}, SharedRb, HeapRb};

type HeapRbRx = Consumer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>;

pub struct AudioRx {
    pub consumer:  HeapRbRx,
}

