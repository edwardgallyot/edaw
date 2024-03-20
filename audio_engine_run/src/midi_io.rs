use std::io::{stdout, Write, stdin};

use anyhow::{Result, anyhow};
use audio_engine::midi_channel::{MidiChannel, MidiPacket};
use midir::{MidiInput, Ignore, MidiInputConnection};

pub struct MidiIo {
    port_index: Option<usize>,
    conn: Option<MidiInputConnection<()>>,
}

impl MidiIo {
    pub fn from_cli() -> Result<MidiIo> {
        let mut input = String::new();
        let mut midi_in = MidiInput::new("some input")?;
        midi_in.ignore(Ignore::None);
        let in_ports = midi_in.ports();
        let port_index;

        match in_ports.len() {
            0 => return Err(anyhow!("no available ports")),
            _ => {
                println!("available input ports: ");
                in_ports
                    .iter()
                    .enumerate()
                    .try_for_each(|(i, p)| -> Result<()> {
                    println!("port {}: {}", i, midi_in.port_name(p)?);
                    Ok(())
                })?;

                print!("select an input port: ");
                stdout().flush()?;
                stdin().read_line(&mut input)?;

                let input: usize = input.trim().parse()?;
     
                let port = in_ports.get(input).ok_or(anyhow!("port doesn't exist"))?;
                port_index = Some(input);
                port.to_owned()
            },
        };

        let midi_io = MidiIo {
            port_index,
            conn: None,
        };

        Ok(midi_io)
    }

    pub fn start(&mut self, engine_in: &mut MidiChannel) -> Result<()> {
        let mut midi_in = MidiInput::new("some input")?;
        midi_in.ignore(Ignore::None);

        let ports = midi_in.ports();

        let port_index =  self
            .port_index
            .ok_or(anyhow!("no port index configured"))?;

        let port = ports
            .get(port_index)
            .ok_or(anyhow!("couldn't find port"))?;

        let mut tx = engine_in
            .take_tx()
            .ok_or(anyhow!("no midi tx"))?;

        let conn = midi_in.connect(
            port,
            "midir-port",
            move |_s, m, _|{
                let message;
                match MidiPacket::from_bytes(m) {
                    Err(e) => {
                        eprintln!("error converting midi packet: {}", e);
                        return;
                    },
                    Ok(m) => {
                        message = m;
                    },
                }
                
                if let Err(e) = tx.push(message) {
                    eprintln!("error pushing message: {e}");
                }
            },
            (),
        ).ok().ok_or(anyhow!("failed to connect"))?;

        self.conn = Some(conn);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.conn
            .take()
            .ok_or(anyhow!("no connection"))?
            .close();

        Ok(())
    }
}
