use std::{net::{TcpStream, Ipv4Addr}, str::FromStr};

use anyhow::{Result, anyhow};

pub struct Connection {
    connection: Option<TcpStream>
}

impl Connection {
    pub fn new() -> Connection {
        let connection = None;
        Connection {
            connection,
        }
    }

    pub fn create_connection(&mut self, addr: &str) -> Result<()> {
        let stream = TcpStream::connect(addr)?;
        self.connection = Some(stream);
        Ok(())
    }

    pub fn shutdown_connection(&mut self) -> Result<()> {
        if let Some(s) = self.connection.take() {
            s.shutdown(std::net::Shutdown::Both)?;
        }
        Err(anyhow!("no connection"))
    }
}
