pub struct State {
    pub counter: usize,
}

#[no_mangle]
pub fn step(state: &mut State) {
    println!("Reload the client");
}
