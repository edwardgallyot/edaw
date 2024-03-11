mod round_robin;
mod note;
mod velocity;

use velocity::Velocity;
use round_robin::RoundRobin;
use note::Note;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct SampleId {
    velocity: Velocity, 
    round_robin: RoundRobin,
    note: Note,
    is_sustained: bool,    
}
