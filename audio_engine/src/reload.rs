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
        match s.start() {
            Err(e) => eprintln!("error starting engine host: {}", e),
            Ok(_) => {},
        }
    }
}

#[no_mangle]
pub fn save(mut state: State) -> State {
    println!("running now");
    if let Some(s) = state.engine_host.take() {
        drop(s);
    }
    state
}
