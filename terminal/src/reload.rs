use crate::terminal::Terminal;

pub struct State {
    pub terminal: Terminal,
}

impl State {
    pub fn new() -> State {
        State {
            terminal: Terminal::new(),
        }
    }
}

#[no_mangle]
pub fn build() -> State {
    State::new()    
}

#[no_mangle]
pub fn save(mut state: State) -> State {
    state.terminal.run.clone().store(false, std::sync::atomic::Ordering::Relaxed);
    if let Some(handle) = state.terminal.handle.take() {
        let _ = handle.join();
    }
    state 
}

#[no_mangle]
pub fn load(state: &mut State)  {
    println!("Reloaded terminal");
    let res = state.terminal.run();
    match res {
        Err(e) => eprintln!("Error running terminal: {}", e),
        Ok(_) => {}
    }
}
