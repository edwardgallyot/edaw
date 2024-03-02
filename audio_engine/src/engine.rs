use crate::audio_channel::AudioChannel;

pub struct AudioEngine {
    audio_in: AudioChannel,
    audio_out: AudioChannel,

    buffer_size: usize,
    num_channels: u32,
}

impl AudioEngine {
    pub fn new(buffer_size: usize, num_channels: u32) -> AudioEngine {
        AudioEngine {
            audio_in: AudioChannel::new(buffer_size * 4),
            audio_out: AudioChannel::new(buffer_size * 4),
            buffer_size,
            num_channels,
        }
    }

    pub fn start() {
        // TODO: Use a new thread here to pop and push the audio samples
    }

    pub fn test(&self) {
        println!("test");
    }
}
