mod screens;
mod connection;

use std::{io::Stdout, ops::ControlFlow};
 
use anyhow::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{backend::CrosstermBackend, widgets::{Paragraph, Widget, block::{Title, Position}, Borders}, style::Stylize, text::{Line, Text}, layout::Alignment, symbols::border};
use ratatui::widgets::*;

use screens::Screen;
use screens::ConnectionAction;

use connection::Connection;


pub struct App {
    counter: u32,
    current_screen: Screen,
    connection: Connection,
    connection_action: Option<ConnectionAction>,
    connection_addr_input: String,
}

impl App {
    pub fn new() -> App {
        let counter = 0;
        let current_screen = Screen::Main;
        let connection = Connection::new();
        let connection_action = None;
        let connection_addr_input = String::new();
        App {
            counter, 
            current_screen,
            connection,
            connection_action,
            connection_addr_input,
        }
    }

    pub fn handle_drawing(
        &self,
        terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(self, frame.size());
        })?;
       Ok(())
    }

    pub fn toggle_connection(&mut self) {
        match self.current_screen {
            Screen::Main => self.current_screen = Screen::Connection,
            Screen::Connection => {
                self.current_screen = Screen::Main;
                self.connection_action = None;
            },
            _ => {},
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        self.connection.create_connection(self.connection_addr_input.as_str())?;
        Ok(())
    }

    pub fn handle_input(&mut self) -> Result<ControlFlow<()>> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                return self.match_key_codes(key.code);
            }
        }
        Ok(ControlFlow::Continue(()))
    }

    fn handle_connection_enter(&mut self) {
        match self.connection_action {
            Some(ConnectionAction::Editing) => self.connection_action = None,
            _ => self.connection_action = Some(ConnectionAction::Editing),
        }
    }

    fn handle_enter(&mut self) {
        match self.current_screen {
            Screen::Connection => {
                self.handle_connection_enter();         
            }
            _ => {}
        }
    }

    fn match_key_codes(&mut self, code: KeyCode) -> anyhow::Result<ControlFlow<()>> {
        match code {
            KeyCode::Char(c) if self.connection_action == Some(ConnectionAction::Editing) => {
                self.connection_addr_input.push(c);
            },
            KeyCode::Backspace if self.connection_action == Some(ConnectionAction::Editing) => {
                let _ = self.connection_addr_input.pop();
            },
            KeyCode::Char('q') => return Ok(ControlFlow::Break(())),
            KeyCode::Char('Q') => return Ok(ControlFlow::Break(())),
            KeyCode::Char('c') => self.toggle_connection(),
            KeyCode::Enter => self.handle_enter(),
            _ => {},
        }
        Ok(ControlFlow::Continue(()))
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {

        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<J>".blue().bold(),
            " Increment ".into(),
            "<K>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let menu_selected = &self.current_screen;

        let counter_line = Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ]);

        let menu_line = Line::from(format!("Currently selected menu: {:?}  ", menu_selected));

        let editing_line = Line::from(self.connection_addr_input.as_str());

        let connection_action_line = Line::from(format!("connection action: {:?}", self.connection_action));

        let counter_text = Text::from(vec![
            counter_line,
            menu_line,
            editing_line,
            connection_action_line,
        ]);

        Paragraph::new(counter_text)
            .block(block)
            .render(area, buf);

    }
}
