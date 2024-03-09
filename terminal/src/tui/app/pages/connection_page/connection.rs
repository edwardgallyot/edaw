use std::{net::{TcpStream, SocketAddr}, time::Duration, str::FromStr};

use anyhow::{anyhow, Result};

pub struct Connection {
    connection: Option<TcpStream>,
}

const CONNECTION_TIMEOUT: Duration = Duration::from_secs(1);

impl Connection {
    pub fn new() -> Connection {
        let connection = None;
        Connection { connection }
    }

    pub fn create_connection(&mut self, addr: &str) -> Result<()> {
        if let None = self.connection {
            let sock_addr = SocketAddr::from_str(addr)?;
            let stream = TcpStream::connect_timeout(&sock_addr, CONNECTION_TIMEOUT)?;
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

impl Drop for Connection {
    fn drop(&mut self) {
        if let Err(e) = self.shutdown_connection() {
            eprintln!("error shutting down connection: {}", e);
        }
    }
}
