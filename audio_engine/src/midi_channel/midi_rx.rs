use std::{mem::MaybeUninit, sync::Arc};
use ringbuf::{Consumer, SharedRb};

use super::midi_packet::MidiPacket;

type HeapRbRx = Consumer<MidiPacket, Arc<SharedRb<MidiPacket, Vec<MaybeUninit<MidiPacket>>>>>;

pub struct MidiRx {
    rx: HeapRbRx,
}

impl MidiRx {
    pub fn new(rx: HeapRbRx) -> MidiRx {
        MidiRx {
            rx,
        }
    }

    pub fn pop(&mut self) -> Option<MidiPacket> {
        self.rx.pop()
    }
}
