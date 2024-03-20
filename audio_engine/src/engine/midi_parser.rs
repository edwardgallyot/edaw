use anyhow::Result;
use edaw_sampler::{Note, Velocity};
use midly::live::LiveEvent;

pub fn print_midi_message(event: &[u8]) -> Result<()> {
    let event = LiveEvent::parse(event)?;

    match event {
        LiveEvent::Midi { channel: _ , message } => match message {
            midly::MidiMessage::NoteOn { key, vel } => {
                println!("note: {:?} at velocity {} ", Note::try_from(key.as_int()), vel.as_int());
            },
            _ => {},
        }
        _ => {},
    };
    Ok(())
}

pub fn convert_midi_for_sampler(event: &[u8]) -> (Option<Note>, Option<Velocity>) {
    let event = match LiveEvent::parse(event) {
        Ok(e) => e,
        Err(_) => {
            return (None, None);
        }
    };
    match event {
        LiveEvent::Midi { channel: _ , message } => match message {
            midly::MidiMessage::NoteOn { key, vel } => {
                let k  = Note::try_from(key.as_int()).ok();
                let v = Velocity::try_from(vel.as_int()).ok();
                return (k, v);
            },
            _ => {},
        }
        _ => {},
    };
    (None, None)
}

