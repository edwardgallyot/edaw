pub struct State {
    pub counter: usize,
}

#[no_mangle]
pub fn load(state: &mut State) {
    println!("Reload the server");
}

#[no_mangle]
pub fn save(state: State) -> State {
    println!("Cool: Save the server");
    state
}
