mod connection_ui;
mod main_ui;

use super::{App, screens::Screen};
use connection_ui::draw_connection_ui;
use main_ui::draw_main_ui;
use ratatui::Frame;

pub fn ui(f: &mut Frame, app: &App) {
    match app.current_screen() {
        Screen::Connection(Some(_)) => draw_connection_ui(f, app),
        _ => draw_main_ui(f, app),
    }
}
