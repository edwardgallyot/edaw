use std::thread::JoinHandle;
use anyhow::Result;

use crate::sampler::sample_id::SampleId;

pub struct SampleLoadTask {
    handle: JoinHandle<Result<()>>,
}

impl SampleLoadTask {
    pub fn load(id: SampleId, path: &str) -> SampleLoadTask {
        todo!()
    }
}
