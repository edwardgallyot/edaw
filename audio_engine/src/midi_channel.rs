mod midi_tx;
mod midi_rx;
mod midi_packet;
mod midi_packet_buffer;
pub use midi_packet::MidiPacket;
pub use midi_packet_buffer::MidiPacketBuffer;

use midi_rx::MidiRx;
use midi_tx::MidiTx;
use ringbuf::HeapRb;


pub struct MidiChannel {
    midi_rx: Option<MidiRx>,
    midi_tx: Option<MidiTx>,
}

impl MidiChannel {
    pub fn new(buffer_size: usize) -> MidiChannel {
        let buf = HeapRb::<MidiPacket>::new(buffer_size);
        let (tx, rx) = buf.split();
        let midi_rx = Some(MidiRx::new(rx));
        let midi_tx = Some(MidiTx::new(tx));
        MidiChannel {
            midi_rx,
            midi_tx,
        }
    }

    pub fn take_tx(&mut self) -> Option<MidiTx> {
        self.midi_tx.take()
    }

    pub fn take_rx(&mut self) -> Option<MidiRx> {
        self.midi_rx.take()
    }
}
