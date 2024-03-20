use std::{
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc,
    },
    thread::{self, JoinHandle},
};

use anyhow::anyhow;

use crate::{audio_channel::AudioChannel, AudioEngine, midi_channel::{MidiChannel, MidiPacket, MidiPacketBuffer}};

pub struct AudioEngineHost {
    pub audio_in: AudioChannel,
    pub audio_out: AudioChannel,

    pub midi_in: MidiChannel,

    buffer_size: usize,
    num_samples_per_channel: usize,
    num_channels: usize,

    run: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,

    midi_buffer: Option<MidiPacketBuffer>,
}

impl AudioEngineHost {
    pub fn new(
        num_samples_per_channel: usize,
        num_channels: usize,
    ) -> anyhow::Result<AudioEngineHost> {
        let buffer_size = num_samples_per_channel * num_channels;

        if buffer_size == 0 {
            return Err(anyhow!("no buffer_size"));
        }

        let audio_in = AudioChannel::new(buffer_size);
        let audio_out = AudioChannel::new(buffer_size);
        let midi_in = MidiChannel::new(buffer_size);
        let run = Arc::new(AtomicBool::default());
        let handle = None;

        let midi_buffer = Some(MidiPacketBuffer::new());

        let host = AudioEngineHost {
            audio_in,
            audio_out,
            midi_in,
            buffer_size,
            num_samples_per_channel,
            num_channels,
            run,
            handle,
            midi_buffer,
        };
        Ok(host)
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let run_clone = self.run.clone();
        run_clone.store(true, Relaxed);

        let mut engine = AudioEngine::new(self.num_samples_per_channel, self.num_channels);

        engine.prepare()?;

        let mut rx = self
            .audio_in
            .take_rx()
            .ok_or(anyhow!("no input rx"))?;

        let mut tx = self
            .audio_out
            .take_tx()
            .ok_or(anyhow!("no output tx"))?;

        let mut midi_rx = self
            .midi_in
            .take_rx()
            .ok_or(anyhow!("no midi rx"))?;

        let mut midi_buffer = self
            .midi_buffer
            .take()
            .ok_or(anyhow!("no midi buffer"))?;

        let mut samples = Vec::new();

        (0..self.buffer_size).for_each(|_| {
            samples.push(0.0);
        });

        let handle = thread::spawn(move || {
            while run_clone.load(Relaxed) {
                // Collect the samples
                rx.collect_samples(samples.as_mut_slice());

                // Collect the midi packets into a midi packet buffer
                while let Some(p) = midi_rx.pop() {
                    if let Err(e) = midi_buffer.push(p) {
                        eprintln!("error pushing to buffer, {}", e);
                        break;
                    }
                }

                engine.process(samples.as_mut_slice(), &mut midi_buffer);
                tx.push_slice(samples.as_mut_slice());
                midi_buffer.clear_buffer();
            }
        });

        self.handle = Some(handle);
        Ok(())
    }
}

impl Drop for AudioEngineHost {
    fn drop(&mut self) {
        self.run.store(false, Relaxed);
        if let Some(h) = self.handle.take() {
            if let Err(e) = h.join() {
                eprintln!("error joining thread: {:?}", e);
            }
        }
    }
}
