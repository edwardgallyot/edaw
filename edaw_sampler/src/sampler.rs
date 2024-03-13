mod sample;
mod sample_id;
mod sample_loader;

use fxhash::FxHashMap;
use sample_id::SampleId;
use sample::Sample;


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
