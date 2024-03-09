use std::default;

use crossterm::event::KeyCode;
use strum::EnumIter;

#[derive(PartialEq, Debug, Default, EnumIter)]
pub enum ConnectStatus {
    #[default]
    ReadyToConnect,
    Connected,
    Error,
}

#[derive(PartialEq, Debug, Default, EnumIter)]
#[repr(u8)]
pub enum ConnectionScreen {
    #[default]
    Edit,
    Editing,
    Connect(ConnectStatus),
    Connecting,
    Disconnect,
    Disconnecting,
}

impl ConnectionScreen {
    pub fn display_string(&self) -> &'static str {
        match self {
            ConnectionScreen::Edit => "Edit",
            ConnectionScreen::Editing => "Editing",
            ConnectionScreen::Connect(ConnectStatus::Connected) => "Connected",
            ConnectionScreen::Connect(ConnectStatus::Error | ConnectStatus::ReadyToConnect) => "Connect",
            ConnectionScreen::Connecting => "Connecting",
            ConnectionScreen::Disconnect => "Disconnect",
            ConnectionScreen::Disconnecting => "Disconnecting",
        }
    }
    pub fn handle_key_press(&mut self, code: &KeyCode) {
        match code {
            KeyCode::Enter => self.handle_enter_key(),
            KeyCode::Char('k') | KeyCode::Up | KeyCode::PageUp => self.handle_up_key(),
            KeyCode::Char('j') | KeyCode::Down | KeyCode::PageDown => self.handle_down_key(),
            _ => {}
        }
    }

    fn handle_enter_key(&mut self) {
        match self {
            ConnectionScreen::Edit => *self = ConnectionScreen::Editing,
            ConnectionScreen::Editing => *self = ConnectionScreen::Edit,
            ConnectionScreen::Connect(ConnectStatus::Error) => *self = ConnectionScreen::Connect(ConnectStatus::ReadyToConnect),
            ConnectionScreen::Connect(ConnectStatus::Connected) => {},
            ConnectionScreen::Connect(ConnectStatus::ReadyToConnect) => *self = ConnectionScreen::Connecting,
            ConnectionScreen::Disconnect => *self = ConnectionScreen::Disconnecting,
            _ => {},
        }
    }

    fn handle_up_key(&mut self) {
        match self {
            ConnectionScreen::Connect(_) => *self = ConnectionScreen::Edit,
            ConnectionScreen::Disconnect => *self = ConnectionScreen::Connect(ConnectStatus::ReadyToConnect),
            _ => {}
        }
    }

    fn handle_down_key(&mut self) {
        match self {
            ConnectionScreen::Edit => *self = ConnectionScreen::Connect(ConnectStatus::ReadyToConnect),
            ConnectionScreen::Connect(_) => *self = ConnectionScreen::Disconnect,
            _ => {}
        }
    }
}
