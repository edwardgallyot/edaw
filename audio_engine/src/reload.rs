use crate::engine::AudioEngineHost;

#[derive(Default)]
pub struct State {
    pub engine_host: Option<AudioEngineHost>,
}

impl State {
    pub fn new() -> State {
        State::default()
    }
}

#[no_mangle]
pub fn build(num_samples_per_channel: usize, num_channels: usize) -> State {
    let mut state = State::new();
    let engine = AudioEngineHost::new(num_samples_per_channel, num_channels);
    state.engine_host = engine.ok();
    state
}

#[no_mangle]
pub fn load(state: &mut State)  {
    if let Some(s) = state.engine_host.as_mut() {
        let _ = s.start();
    }
}

#[no_mangle]
pub fn save(state: State) -> State {
    state
}
