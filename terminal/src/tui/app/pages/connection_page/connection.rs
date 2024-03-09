use std::net::TcpStream;

use anyhow::{anyhow, Result};

pub struct Connection {
    connection: Option<TcpStream>,
}

impl Connection {
    pub fn new() -> Connection {
        let connection = None;
        Connection { connection }
    }

    pub fn create_connection(&mut self, addr: &str) -> Result<()> {
        if let None = self.connection {
            let stream = TcpStream::connect(addr)?;
            self.connection = Some(stream);
        } else {
            return Err(anyhow!("already exists"));
        }
        Ok(())
    }

    pub fn shutdown_connection(&mut self) -> Result<()> {
        if let Some(s) = self.connection.take() {
            s.shutdown(std::net::Shutdown::Both)?;
        }
        Err(anyhow!("no connection"))
    }

    pub fn exists(&self) -> bool {
        match self.connection {
            Some(_) => true,
            None => false,
        }
    }
}
