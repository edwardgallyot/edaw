use std::sync::Arc;

mod atomic_state;

use atomic_state::AtomicState;

#[derive(Default)]
pub struct State {
    state: Arc<AtomicState>,
}

impl State {
    pub fn new() -> State {
        State::default()
    }
}
