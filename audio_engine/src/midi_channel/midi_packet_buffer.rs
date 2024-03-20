use anyhow::{Result, anyhow};

use super::MidiPacket;

const MAX_MIDI_PACKETS_PER_BLOCK: usize = 64;
const MIDI_PACKET_NONE: Option<MidiPacket> = None;

// A nice stack allocated buffer for midi packets
pub struct MidiPacketBuffer {
    buffer: [Option<MidiPacket>; MAX_MIDI_PACKETS_PER_BLOCK],
    index: usize,
}

impl MidiPacketBuffer {
    pub fn new() -> MidiPacketBuffer {
        let buffer = [MIDI_PACKET_NONE; MAX_MIDI_PACKETS_PER_BLOCK];
        let index = 0;
        MidiPacketBuffer {
            buffer,
            index,
        }
    }

    pub fn clear_buffer(&mut self) {
        self.buffer
            .iter_mut()
            .for_each(|p| {
                p.take();
            });
        self.index = 0;
    }

    pub fn push(&mut self, packet: MidiPacket) -> Result<()> {
        let p = self
            .buffer
            .get_mut(self.index)
            .ok_or(anyhow!("no room"))?;
        *p = Some(packet);
        self.index += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<MidiPacket> {
        if let Some(p) = self.buffer.get_mut(self.index) {
            if let Some(i) = self.index.checked_sub(1) {
                self.index = i;
            }
            // This could be none if the index is 0 and the last 
            // packet has been popped. It's not an error we just
            // let the caller decide.
            p.take() 
        } else {
            eprintln!("index out of range");
            None
        }
    }

    pub fn packets(&self) -> &[Option<MidiPacket>] {
        &self.buffer[0..self.index]
    }
}
