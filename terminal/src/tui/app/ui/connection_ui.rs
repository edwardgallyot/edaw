use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{layout::Layout, Frame};
use strum::IntoEnumIterator;

use crate::tui::app::screens::{ConnectionScreen, ConnectStatus};

use super::{draw_ui_title, get_inner_area, App};

const BLOCK_HEIGHT: u16 = 3;

pub fn draw_connection_ui(f: &mut Frame, app: &App, screen: &ConnectionScreen) {
    draw_ui_title(f, "Connection Page");

    let mut inner = get_inner_area(f);

    let mut layout = vec![];
    let mut paragraphs = vec![];

    inner.height = 0;

    ConnectionScreen::iter().for_each(|c| {
        match c {
            ConnectionScreen::Editing
            | ConnectionScreen::Connecting
            | ConnectionScreen::Disconnecting => return,
            _ => {}
        };

        layout.push(ratatui::layout::Constraint::Max(BLOCK_HEIGHT));

        let bg_color = match c {
            ConnectionScreen::Edit => {
                match *screen {
                    ConnectionScreen::Edit => Color::LightGreen,
                    ConnectionScreen::Editing => Color::LightBlue,
                    _ => Color::Black,
                }
            },
            _ if *screen == c => Color::LightGreen,
            _ => Color::Black,
            
        };

        let fg_color = match c {
            ConnectionScreen::Edit => {
                match *screen {
                    ConnectionScreen::Edit | ConnectionScreen::Editing => Color::Black,
                    _ => Color::White,
                }
            },
            _ if *screen == c => Color::Black,
            _ => Color::White,
            
        };

        let block = Block::new()
            .borders(Borders::ALL)
            .bg(bg_color)
            .fg(fg_color);

        inner.height += BLOCK_HEIGHT;

        let display_string = match *screen {
            ConnectionScreen::Editing => {
                match c {
                    ConnectionScreen::Edit => app.connection_page().input_string(),
                    _ => c.display_string(),
                }
            },
            ConnectionScreen::Connect(ConnectStatus::Connected) => {
                match c {
                    ConnectionScreen::Connect(_) => "Connected",
                    _ => c.display_string(),
                }
            },
            ConnectionScreen::Connect(ConnectStatus::Error) => {
                match c {
                    ConnectionScreen::Connect(_) => {
                        if let Some(s) = app.connection_page().error_string() {
                            s.as_str()
                        } else {
                            c.display_string()
                        }
                    } 
                    _ => c.display_string()
                }
            }
            _ if !app.connection_page().input_string().is_empty() => {
                match c {
                    ConnectionScreen::Edit => app.connection_page().input_string(),
                    _ => c.display_string(),
                }
            }
            _ => c.display_string(),
        };

        let paragraph = Paragraph::new(display_string)
            .alignment(ratatui::layout::Alignment::Center)
            .block(block);

        paragraphs.push(paragraph);
    });

    let rows = Layout::default()
        .constraints(layout)
        .direction(ratatui::layout::Direction::Vertical)
        .split(inner);

    rows.iter().enumerate().for_each(|(i, r)| {
        if let Some(p) = paragraphs.get(i) {
            f.render_widget(p.clone(), *r);
        }
    });
}
