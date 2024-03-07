mod ping;
pub use ping::*;
use serde::{Serialize, Deserialize};

use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Message {
    Ping(Ping),
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> Result<Message> {
        match bincode::deserialize(bytes) {
            Err(e) => Err(anyhow!("error deserialising message: {}", e)),
            Ok(m) => Ok(m),
        }
    }
    pub fn to_bytes(&mut self) -> Result<Vec<u8>> {
        match bincode::serialize(&self) {
            Err(e) => Err(anyhow!("error serialising message: {}", e)),
            Ok(b) => Ok(b),
        }
    }
}
