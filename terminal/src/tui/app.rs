use std::sync::{atomic::AtomicBool, Arc};

#[derive(Default)]
pub struct App {
    pub counter: i32,
}

impl App {
    pub fn new() -> App {
        App::default()
    }
}
