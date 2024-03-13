use anyhow::{Result, anyhow};
use midir::{MidiInput, Ignore};

pub struct MidiIo {


}

impl MidiIo {
    pub fn from_cli() -> Result<MidiIo> {
        let mut input = String::new();
        let mut midi_in = MidiInput::new("some input")?;
        midi_in.ignore(Ignore::None);

        let in_ports = midi_in.ports();

        let in_port = match in_ports.len() {
            0 => return Err(anyhow!("no available ports")),
            _ => {
                for (i, _port) in in_ports.iter().enumerate() {
                    println!("port: {}", i);
                } 
            },
        };

        let midi_io = MidiIo {
            
        };
        Ok(midi_io)
    }
}
