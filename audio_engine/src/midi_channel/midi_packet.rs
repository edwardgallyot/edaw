use anyhow::{Result, anyhow};

// We want midi packets as our own type to send around
// since we know this is stack allocated and will work
// with ffi slices if we want to integrate with JUCE.
#[derive(Debug)]
pub struct MidiPacket {
    bytes: [u8; 3],
}

impl MidiPacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<MidiPacket> {
        match bytes.len() {
            3 => {
                let bytes = bytes.try_into()?;
                return Ok(MidiPacket{
                    bytes
                })
            },
            _ => { return Err(anyhow!("unknown size"))}
        };
       
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}
