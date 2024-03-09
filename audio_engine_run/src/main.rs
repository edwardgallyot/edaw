mod audio_io;

use hot_lib_reloader::*;

#[hot_module(
    dylib = "audio_engine",
    lib_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug")
)]
mod hot_lib {
    hot_functions_from_file!("audio_engine/src/reload.rs");

    use audio_engine::State;

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

fn main() {
    let lib_observer = hot_lib::subscribe();
    let mut audio_io = audio_io::AudioIo::new();

    if let Err(e) = audio_io.configure() {
        eprintln!("Error configuring I/O: {}", e);
        return;
    }

    loop {
        if audio_io.get_num_samples_per_channel() == 0 {
            eprintln!("no samples");
            return;
        }

        if audio_io.get_num_channels() == 0 {
            eprintln!("no channels");
            return;
        }

        let mut state = hot_lib::build(
            audio_io.get_num_samples_per_channel(),
            audio_io.get_num_channels(),
        );

        hot_lib::load(&mut state);

        if let Some(s) = &mut state.engine_host {
            let mut engine_in = &mut s.audio_in;
            let mut engine_out = &mut s.audio_out;
            if let Err(e) = audio_io.start(&mut engine_in, &mut engine_out) {
                eprintln!("Error starting the audio engine: {}", e);
                return;
            }
        }

        lib_observer.wait_for_about_to_reload();

        if let Err(e) = audio_io.stop() {
            eprintln!("Error stopping the audio engine: {}", e);
        }

        hot_lib::save(state);

        lib_observer.wait_for_reload();
    }
}
