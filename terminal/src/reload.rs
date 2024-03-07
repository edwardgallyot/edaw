use crate::tui::Tui;

#[derive(Default)]
pub struct State {
    pub tui: Option<Tui>,
}

impl State {
    pub fn new() -> State {
        State::default()
    }
}

#[no_mangle]
pub fn build() -> State {
    let mut state = State::new();
    let tui = Tui::new();
    state.tui = Some(tui);
    state
}

#[no_mangle]
pub fn save(mut state: State) -> State {
    if let Some(s) = state.tui.take() {
        drop(s);
    }
    state
}

#[no_mangle]
pub fn load(state: &mut State) {
    if let Some(s) = state.tui.as_mut() {
        match s.run() {
            Err(e) => eprintln!("error running terminal: {}", e),
            Ok(_) => {}
        }
    }
}
