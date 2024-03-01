use crate::tui::Tui;

pub struct State {
    pub tui: Tui,
}

impl State {
    pub fn new() -> State {
        State { tui: Tui::new() }
    }
}

#[no_mangle]
pub fn build() -> State {
    State::new()
}

#[no_mangle]
pub fn save(mut state: State) -> State {
    state.tui.stop();
    state
}

#[no_mangle]
pub fn load(state: &mut State) {
    println!("Reloaded terminal");
    let res = state.tui.run();
    match res {
        Err(e) => eprintln!("Error running terminal: {}", e),
        Ok(_) => {}
    }
}
