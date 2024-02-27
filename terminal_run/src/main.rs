#[hot_lib_reloader::hot_module(
    dylib = "terminal",
    lib_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug")
)]
mod hot_lib {
    hot_functions_from_file!("terminal/src/ffi.rs");

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}

    pub use terminal::State;
}

fn main() {
    let mut state = hot_lib::State { counter: 0 };

    let lib_observer = hot_lib::subscribe();

    loop {
        hot_lib::step(&mut state);
        lib_observer.wait_for_reload();
    }
}
