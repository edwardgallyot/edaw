mod connection_ui;
mod main_ui;

use super::{screens::Screen, App};
use connection_ui::draw_connection_ui;
use main_ui::draw_main_ui;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders},
    Frame,
};

pub const INNER_OFFSET: u16 = 3;

pub fn draw_ui_title(f: &mut Frame, title: &str) {
    let title = Title::from(title).alignment(ratatui::layout::Alignment::Center);
    let title = Title::from(title).alignment(ratatui::layout::Alignment::Center);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default());

    f.render_widget(title_block, f.size());
}

pub fn get_inner_area(f: &mut Frame) -> Rect {
    let mut inner_area = f.size();

    inner_area.x += INNER_OFFSET;
    inner_area.y += INNER_OFFSET;
    inner_area.width -= INNER_OFFSET * 2;
    inner_area.height -= INNER_OFFSET * 2;

    inner_area
}

pub fn ui(f: &mut Frame, app: &App) {
    match app.current_screen() {
        Screen::Connection(Some(c)) => draw_connection_ui(f, app, c),
        _ => draw_main_ui(f, app),
    }
}
