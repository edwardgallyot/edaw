use crate::engine::AudioEngine;

#[derive(Default)]
pub struct State {
    pub engine: Option<AudioEngine>,
}

impl State {
    pub fn new() -> State {
        State::default()
    }

    pub fn register_audio_engine(&mut self, engine: AudioEngine) {
        self.engine = Some(engine);
    }
}

#[no_mangle]
pub fn load(_state: &mut State) {
    println!("Reload the server");
    
}

#[no_mangle]
pub fn save(state: State) -> State {
    println!("Cool: Save the server");
    state
}
