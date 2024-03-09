mod connection_screen;

pub use connection_screen::*;
use crossterm::event::KeyCode;
use strum::EnumIter;

// This is a big nested enum for handling the selected screen,
// One screen should map to one entry in this enum with any subscreen
// being the value of the variant.
#[derive(PartialEq, Debug, Default, EnumIter)]
#[repr(u8)]
pub enum MainScreen {
    #[default]
    Main,
    Connection,
    Samples,
}

#[derive(PartialEq, Debug, Default, EnumIter)]
#[repr(u8)]
pub enum Screen {
    #[default]
    Main,
    Connection(Option<ConnectionScreen>),
    Samples,
}

impl Screen {
    pub fn get_display_string(&self) -> &'static str {
        match self {
            Screen::Main => "Main",
            Screen::Connection(_) => "Connection",
            Screen::Samples => "Samples",
        }
    }

    pub fn handle_key_press(&mut self, code: &KeyCode) {
        match self {
            Screen::Main | Screen::Connection(None) | Screen::Samples => {
                self.handle_main_screen(code)
            }

            Screen::Connection(Some(c)) => {
                match code {
                    KeyCode::Esc | KeyCode::Char('q') => *self = Screen::Connection(None),
                    _ => c.handle_key_press(code),
                };
            }
        }
    }

    fn handle_main_screen(&mut self, code: &KeyCode) {
        match code {
            KeyCode::Char('k') | KeyCode::Up | KeyCode::PageUp => self.handle_up_key(),
            KeyCode::Char('j') | KeyCode::Down | KeyCode::PageDown => self.handle_down_key(),
            KeyCode::Enter => self.handle_enter_key(),
            _ => {}
        }
    }

    fn handle_enter_key(&mut self) {
        match self {
            Screen::Connection(None) => {
                *self = Screen::Connection(Some(ConnectionScreen::default()))
            }
            _ => {}
        }
    }

    fn handle_up_key(&mut self) {
        match self {
            Screen::Connection(_) => *self = Screen::Main,
            Screen::Samples => *self = Screen::Connection(None),
            _ => {}
        }
    }

    fn handle_down_key(&mut self) {
        match self {
            Screen::Main => *self = Screen::Connection(None),
            Screen::Connection(_) => *self = Screen::Samples,
            _ => {}
        }
    }
}
