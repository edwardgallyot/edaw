use std::{mem::MaybeUninit, sync::Arc};
use anyhow::{Result, anyhow};
use ringbuf::{Producer, SharedRb};

use super::midi_packet::MidiPacket;

type HeapRbTx = Producer<MidiPacket, Arc<SharedRb<MidiPacket, Vec<MaybeUninit<MidiPacket>>>>>;

pub struct MidiTx {
    tx: HeapRbTx,
}

impl MidiTx {
    pub fn new(tx: HeapRbTx) -> MidiTx {
        MidiTx {
            tx,
        }
    }

    pub fn push(&mut self, message: MidiPacket) -> Result<()> {
        if let Err(e) = self.tx.push(message) {
            return Err(anyhow!("couldnt' push message: {:?}", e));
        }
        Ok(())
    }
}
