use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{layout::Layout, Frame};
use strum::IntoEnumIterator;

use crate::tui::app::screens::ConnectionScreen;

use super::{draw_ui_title, get_inner_area, App};

const BLOCK_HEIGHT: u16 = 3;

pub fn draw_connection_ui(f: &mut Frame, _app: &App, screen: &ConnectionScreen) {
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

        let color = if *screen == c {
            Color::LightGreen
        } else {
            Color::Black
        };

        let block = Block::new().borders(Borders::ALL).bg(color);

        inner.height += BLOCK_HEIGHT;

        let paragraph = Paragraph::new(c.display_string())
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
