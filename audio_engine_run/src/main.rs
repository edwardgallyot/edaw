mod audio_io;

use audio_engine::AudioEngine;
use hot_lib_reloader::*;

#[hot_module(
    dylib = "audio_engine",
    lib_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug")
)]

mod hot_lib {
    hot_functions_from_file!("audio_engine/src/reload.rs");

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}

    pub use audio_engine::State;
}

fn main() {
    let mut state = hot_lib::State::new();
    let lib_observer = hot_lib::subscribe();
    let mut audio_io = audio_io::AudioIo::new();

    if let Err(e) = audio_io.configure() {
        eprintln!("Error configuring I/O: {}", e);
        return;
    }

    loop {

        let engine = AudioEngine::new(2048);

        hot_lib::load(&mut state);

        if let Err(e) = audio_io.start() {
            eprintln!("Error starting the audio engine: {}", e);
            return;
        }

        lib_observer.wait_for_about_to_reload();

        if let Err(e) = audio_io.stop() {
            eprintln!("Error stopping the audio engine: {}", e);
        }

        state = hot_lib::save(state);

        lib_observer.wait_for_reload();
    }
}
