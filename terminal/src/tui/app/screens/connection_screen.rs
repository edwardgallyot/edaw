use strum::EnumIter;
use crossterm::event::KeyCode;

#[derive(PartialEq, Debug, Default, EnumIter)]
#[repr(u8)]
pub enum ConnectionScreen {
    #[default]
    Edit,
    Editing,
    Connect,
    Connecting,
    Disconnect,
    Disconnecting,
}

impl ConnectionScreen {
    pub fn handle_key_press(&mut self, code: &KeyCode) {
        match code {
            KeyCode::Enter => self.handle_enter_key(),
            KeyCode::Char('k') |
            KeyCode::Up |
            KeyCode::PageUp 
                => self.handle_up_key(),
            KeyCode::Char('j') |
            KeyCode::Down |
            KeyCode::PageDown 
                => self.handle_down_key(),
            _ => {},
        }
    }

    fn handle_enter_key(&mut self) {
        match self {
            ConnectionScreen::Edit => *self = ConnectionScreen::Editing,
            ConnectionScreen::Editing => *self = ConnectionScreen::Edit,
            ConnectionScreen::Connect => *self = ConnectionScreen::Connecting,
            ConnectionScreen::Connecting => *self = ConnectionScreen::Connect,
            ConnectionScreen::Disconnect => *self = ConnectionScreen::Disconnecting,
            ConnectionScreen::Disconnecting => *self = ConnectionScreen::Disconnect,
        }
    }

    fn handle_up_key(&mut self) {
        match self {
            ConnectionScreen::Connect => *self = ConnectionScreen::Edit,
            ConnectionScreen::Disconnect => *self = ConnectionScreen::Connect,
            _ => {}
        }
    }

    fn handle_down_key(&mut self) {
        match self {
            ConnectionScreen::Edit => *self = ConnectionScreen::Connect,
            ConnectionScreen::Connect => *self = ConnectionScreen::Disconnect,
            _ => {}
        }
    }
}
