mod audio_io;
mod midi_io;

use anyhow::{Result, anyhow};
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

fn run() -> Result<()> {
    let lib_observer = hot_lib::subscribe();

    let mut audio_io = audio_io::AudioIo::from_cli()?;
    let mut midi_io = midi_io::MidiIo::from_cli()?;

    loop {
        midi_io.start()?;
        if audio_io.get_num_samples_per_channel() == 0 {
            return Err(anyhow!("no samples"));
        }

        if audio_io.get_num_channels() == 0 {
            return Err(anyhow!("no channels"));
        }

        let mut state = hot_lib::build(
            audio_io.get_num_samples_per_channel(),
            audio_io.get_num_channels(),
        );

        hot_lib::load(&mut state);

        if let Some(s) = &mut state.engine_host {
            let mut engine_in = &mut s.audio_in;
            let mut engine_out = &mut s.audio_out;
            audio_io.start(&mut engine_in, &mut engine_out)?;
        }

        lib_observer.wait_for_about_to_reload();


        lib_observer.wait_for_reload();

        audio_io.stop()?;
        midi_io.stop()?;

        hot_lib::save(state);
    }

}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error running app: {}, try again...", e);
    }
}
