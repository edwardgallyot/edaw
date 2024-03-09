mod audio_rx;
mod audio_tx;

pub use audio_tx::AudioTx;
use ringbuf::HeapRb;

pub use audio_rx::AudioRx;

pub struct AudioChannel {
    // Because these channels are made
    // to be used across threads they are options
    // in the sense that another can thread can
    // take the option and leave None in it's place.
    audio_rx: Option<AudioRx>,
    audio_tx: Option<AudioTx>,
}

impl AudioChannel {
    pub fn new(buffer_size: usize) -> Self {
        let ring = HeapRb::<f32>::new(buffer_size);
        let (producer, consumer) = ring.split();

        let audio_rx = Some(AudioRx::new(consumer));
        let audio_tx = Some(AudioTx::new(producer));

        Self { audio_rx, audio_tx }
    }

    pub fn take_tx(&mut self) -> Option<AudioTx> {
        self.audio_tx.take()
    }

    pub fn take_rx(&mut self) -> Option<AudioRx> {
        self.audio_rx.take()
    }
}
