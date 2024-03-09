/* In order to make the logic simpler we divide our logic into:
* screens: this is how we navigate around the app
* pages: defines the behaviour of screen once you've navigated there
* ui: the actual drawing logic
*/
mod pages;
mod screens;
mod ui;

use std::{io::Stdout, ops::ControlFlow};

use anyhow::Result;
use crossterm::event::{self, KeyCode};
use pages::ConnectionPage;
use ratatui::backend::CrosstermBackend;
use screens::Screen;
use ui::ui;

pub struct App {
    current_screen: Screen,
    connection_page: ConnectionPage,
}

impl App {
    pub fn new() -> App {
        let current_screen = Screen::Main;
        let connection_page = ConnectionPage::new();
        App {
            current_screen,
            connection_page,
        }
    }

    pub fn current_screen(&self) -> &Screen {
        &self.current_screen
    }

    pub fn handle_drawing(
        &self,
        terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<()> {
        terminal.draw(|frame| {
            ui(frame, self);
        })?;
        Ok(())
    }

    pub fn handle_input(&mut self) -> Result<ControlFlow<()>> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                return self.handle_key_event(key.code);
            }
        }
        Ok(ControlFlow::Continue(()))
    }

    fn handle_key_event(&mut self, code: KeyCode) -> anyhow::Result<ControlFlow<()>> {
        // Update the screen selection
        self.current_screen.handle_key_press(&code);

        // Do something based on the current selection
        match self.current_screen {
            Screen::Main => {}
            Screen::Connection(None) => {}
            Screen::Connection(Some(ref mut c)) => {
                self.connection_page.handle_current_screen(&code, c)
            }
            Screen::Samples => {}
        }
        return Ok(ControlFlow::Continue(()));
    }
}
