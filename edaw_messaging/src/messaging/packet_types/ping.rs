use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Ping {
    time: u64,
}

impl Ping {
    pub fn new(time: u64) -> Ping {
        Ping {
            time,
        } 
    }
}

