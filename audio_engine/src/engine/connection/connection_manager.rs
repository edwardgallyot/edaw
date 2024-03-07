use std::{time::Duration, io::{self, Read}, str::from_utf8, ops::ControlFlow};

use anyhow::{Result, anyhow};
use edaw_messaging::{MessageTx, Message, Ping};
use fxhash::FxHashMap;
use mio::{Token, Poll, Events, net::TcpListener, net::TcpStream, Interest, event::Event, Registry};

const SERVER: Token = Token(0);
const RX_BUFFER_SIZE: usize = 4096;

pub enum RxStatus {
    Recieved(u32),
    Nothing
}

pub struct ConnectionManager {
    poll: Poll, 
    events: Events,
    server: TcpListener,
    map: FxHashMap<Token, TcpStream>,
    next_available_client: Token,
    tx: MessageTx,
}


impl ConnectionManager {
    pub fn new(tx: MessageTx) -> Result<ConnectionManager> {
        let poll = Poll::new()?;
        let events = Events::with_capacity(128);

        let addr = "127.0.0.1:9000".parse()?;
        let mut server = TcpListener::bind(addr)?;

        poll.registry()
            .register(&mut server, SERVER, Interest::READABLE)?;

        let map = FxHashMap::default();

        let mut token = Token(SERVER.0 + 1);
        let next_available_client = ConnectionManager::get_next_token(&mut token);

        let connection_manager = ConnectionManager {
            poll,
            events,
            server,
            map,
            next_available_client,
            tx,
        };

        Ok(connection_manager)
    }

    pub fn accept_connections(&mut self) -> Result<()> {
        let timeout = Duration::from_secs(1);

        if let Err(e) = self.poll.poll(&mut self.events, Some(timeout)) {
            if interrupted(&e) {
                return Ok(());
            }
            eprintln!("error polling: {}", e);
        }

        let ConnectionManager {
            poll,
            events,
            server,
            map,
            next_available_client,
            tx,
        } = self;

        for event in events.iter(){
            ConnectionManager::handle_event(
                poll,
                event,
                server,
                map,
                next_available_client,
                tx,
            )?;
        }

        Ok(())
    }

    fn handle_event(
        poll: &mut Poll,
        event: &Event,
        server: &mut TcpListener,
        map: &mut FxHashMap<Token, TcpStream>,
        next_available_client: &mut Token, 
        tx: &mut MessageTx,
    ) -> Result<()> {
        match event.token() {
            SERVER => {
                ConnectionManager::handle_server_event(
                poll,
                server,
                map,
                next_available_client,
                )?;
            }
            token => {
                ConnectionManager::handle_client_event(
                    token,
                    poll,
                    event,
                    map,
                    tx,
                )?;
            }
        }
        Ok(())
    }

    fn handle_server_event(
        poll: &mut Poll,
        server: &mut TcpListener,
        map: &mut FxHashMap<Token, TcpStream>,
        next_available_client: &mut Token, 
    ) -> Result<()> {
        loop {
            let (mut connection, address) = match server.accept() {
                Ok((connection, address)) => (connection, address),
                Err(e) if would_block(&e) => {
                    return Ok(());
                }
                Err(e) => {
                    return Err(anyhow!("error accepting socket: {}", e));
                }
            };

            println!("Accepted connection from : {}", address);

            let token = ConnectionManager::get_next_token(next_available_client);
            poll.registry().register(
                &mut connection,
                token,
                Interest::READABLE.add(Interest::WRITABLE)
            )?;

            map.insert(token, connection);
        }
    }

    fn get_next_token(current: &mut Token) -> Token {
        let next = current.0;
        current.0 += 1;
        Token(next)
    }


    fn handle_client_event(
        token: Token,
        poll: &mut Poll,
        event: &Event,
        map: &mut FxHashMap<Token, TcpStream>,
        tx: &mut MessageTx,
    ) -> Result<()> {
        let done = if let Some(connection) = map.get_mut(&token) {
            ConnectionManager::handle_client_connection_event(
                poll.registry(),
                connection,
                event,
                tx,
            )?
        } else {
            false
        };
        if done {
            if let Some(mut connection) = map.remove(&token) {
                poll.registry().deregister(&mut connection)?;
            }
        }
        Ok(())
    }

    fn handle_client_connection_event(
        _registry: &Registry,
        connection: &mut TcpStream,
        event: &Event,
        tx: &mut MessageTx,
    ) -> Result<bool> {
        // TODO: remove this...
        let test_message = Message::Ping(Ping::new(10));
        tx.send(test_message)?;

        if event.is_readable() {
            return ConnectionManager::handle_client_readable_connection_event(
                connection,
            );
        }
        
        Ok(false)
    }

    fn handle_client_readable_connection_event(
        connection: &mut TcpStream
    ) -> Result<bool> {
        let mut connection_closed = false;
        let mut bytes_read = 0;
        let mut rx_buffer = [0; RX_BUFFER_SIZE];

        let status;
        loop {
            if let ControlFlow::Break(s) = ConnectionManager::recv(connection, &mut rx_buffer)? {
                status = s;
                break;
            } else {
                continue;
            }
        }


        if let RxStatus::Recieved(s) = status {
            if s == 0 {
                connection_closed = true;
            } else {
                bytes_read = s;
            }
        }


        if bytes_read != 0 {
            let rx_data = &rx_buffer[..bytes_read as usize];

            if let Ok(str_buf) = from_utf8(rx_data) {
                println!("recieved: {}", str_buf.trim());
            } else {
                println!("recieved nada");
            }
        }

        if connection_closed {
            println!("Closing connection...");
            return Ok(true)
        }

        return Ok(false)

    }

    fn recv(connection: &mut TcpStream, rx_buffer: &mut [u8; RX_BUFFER_SIZE]) -> Result<ControlFlow<RxStatus>>  {
        match connection.read(rx_buffer) {
            Ok(n) => {
                return Ok(ControlFlow::Break(RxStatus::Recieved(n as u32)));
            } 
            Err(ref err) if would_block(err) => {
                return Ok(ControlFlow::Break(RxStatus::Nothing)); 
            }
            Err(ref err) if interrupted(err) => {
                return Ok(ControlFlow::Continue(())); 
            }
            Err(err) => return Err(anyhow!("error handling client connection: {}", err)),
        }
    }
}


fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
