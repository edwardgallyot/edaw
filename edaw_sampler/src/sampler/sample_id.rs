mod round_robin;
mod note;
mod velocity;

use round_robin::RoundRobin;

// These guys are public because we'll need to convert to and from u8s
// when parsing midi messages
pub use note::Note;
pub use velocity::Velocity;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct SampleId {
    velocity: Velocity, 
    round_robin: RoundRobin,
    note: Note,
    is_sustained: bool,
}


