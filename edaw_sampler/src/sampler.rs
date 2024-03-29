mod sample;
mod sample_id;
mod sample_loader;

use fxhash::FxHashMap;
use sample_id::SampleId;
use sample::Sample;

// These guys are public because we'll need to convert to and from u8s
// when parsing midi messages
pub use sample_id::Note;
pub use sample_id::Velocity;

pub struct Sampler {
    num_samples_per_channel: usize,
    num_channels: usize,
    samples: FxHashMap<SampleId, Sample>
}

impl Sampler {
    pub fn new(num_samples_per_channel: usize, num_channels: usize) -> Sampler {
        let samples = FxHashMap::default();
        Sampler {
            num_samples_per_channel,
            num_channels,
            samples,
        }
    }

    pub fn start_load_task(&mut self) {

    }

    pub fn next_block(&self, data: &mut [f32]) {

    }
}
