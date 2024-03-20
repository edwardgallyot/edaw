#![allow(dead_code)]

// TODO: In the future it'd be nice to have all the midi
// velocities represented here and then allow velocity groups
// to be formed based on a range of 1-127 velocities.

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Velocity {
    V1,
    V2,
    V3,
    V4,
    V5,
}

// TODO: try_from would be better than panicking
impl TryFrom<u8> for Velocity {
    type Error = &'static str; 
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Velocity::*;
        match value {
            0..=25 => Ok(V1),
            26..=50 => Ok(V2),
            51..=80 => Ok(V3),
            81..=100 => Ok(V4),
            101..=127 => Ok(V5),
            _ => Err("invalid value"),
        }
    }
}
