pub struct State {
    pub counter: usize,
}

#[no_mangle]
pub fn step(state: &mut State) {
    state.counter += 1;
    println!("Yeah let's reload!");
}
