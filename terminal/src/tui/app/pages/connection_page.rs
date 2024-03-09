mod connection;

use crate::tui::app::screens::{ConnectionScreen, ConnectStatus};
use connection::Connection;
use crossterm::event::KeyCode;

pub struct ConnectionPage {
    connection: Connection,
    addr_input: String,
    error_string: Option<String>,
}

impl ConnectionPage {
    pub fn new() -> ConnectionPage {
        let connection = Connection::new();
        let addr_input = String::new();
        let error_string = None;

        ConnectionPage {
            connection,
            addr_input,
            error_string,
        }
    }

    pub fn handle_current_screen(&mut self, code: &KeyCode, screen: &mut ConnectionScreen) {
        match screen {
            ConnectionScreen::Editing => {
                self.handle_editing_screen(code);
            }
            ConnectionScreen::Connecting => {
                self.handle_connecting_screen(screen);
            }
            ConnectionScreen::Disconnecting => {
                self.handle_disconnecting_screen(screen);
            }
            _ => {}
        };
    }

    pub fn is_connection_active(&self) -> bool {
        self.connection.exists()
    }

    pub fn input_string(&self) -> &String {
        &self.addr_input
    }

    pub fn error_string(&self) -> Option<&String> {
        self.error_string.as_ref()
    }

    pub fn clear_error_string(&mut self) {
        self.error_string = None;
    }

    fn handle_editing_screen(&mut self, code: &KeyCode) {
        if let KeyCode::Char(c) = code {
            self.addr_input.push(*c);
        }
        if let KeyCode::Backspace = code {
            let _ = self.addr_input.pop();
        }
    }

    fn handle_connecting_screen(&mut self, screen: &mut ConnectionScreen) {
        if let Err(e) = self.connection.create_connection(&self.addr_input) {
            self.error_string = Some(e.to_string());
            *screen = ConnectionScreen::Connect(ConnectStatus::Error);
        } else {
            *screen = ConnectionScreen::Connect(ConnectStatus::Connected);
        }
    }

    fn handle_disconnecting_screen(&mut self, screen: &mut ConnectionScreen) {
        if let Err(e) = self.connection.shutdown_connection() {
            self.error_string = Some(e.to_string());
        }
        *screen = ConnectionScreen::Disconnect;
    }
}
