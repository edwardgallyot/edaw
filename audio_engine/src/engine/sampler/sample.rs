pub struct Sample {
    sample_index: u32,
    num_channels: u32,
    samples: Vec<f32>
}

impl Sample {
    pub fn new(sample_index: u32, num_channels: u32, samples: Vec<f32>) -> Sample {
        Sample {
            sample_index,
            num_channels,
            samples,
        }
    }

    pub fn from_path(path: &str) -> Sample {
        todo!()
    }
}
