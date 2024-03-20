use anyhow::Result;
use edaw_sampler::Note;
use midly::live::LiveEvent;

pub fn print_midi_message(event: &[u8]) -> Result<()> {
    let event = LiveEvent::parse(event)?;

    match event {
        LiveEvent::Midi { channel: _ , message } => match message {
            midly::MidiMessage::NoteOn { key, vel } => {
                println!("note: {:?} at velocity {} ", Note::from(key.as_int()), vel.as_int());
            },
            _ => {},
        }
        _ => {},
    };
    Ok(())
}

pub fn convert_midi_for_sampler(event: &[u8]) -> Option<Note> {
    let event = LiveEvent::parse(event).ok()?;
    match event {
        LiveEvent::Midi { channel: _ , message } => match message {
            midly::MidiMessage::NoteOn { key, vel: _ } => {
                return Some(Note::from(key.as_int()));
            },
            _ => {},
        }
        _ => {},
    };
    None
}

