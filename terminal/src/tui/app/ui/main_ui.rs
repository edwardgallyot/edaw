use super::{super::screens::Screen, draw_ui_title, get_inner_area, App};

use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
    Frame,
};
use strum::IntoEnumIterator;

pub fn draw_main_ui(f: &mut Frame, app: &App) {
    draw_ui_title(f, "Edaw Terminal");

    let inner_area = get_inner_area(f);

    let mut constraints = vec![];
    for _ in Screen::iter() {
        constraints.push(Constraint::Length(4))
    }

    let mut list_items = vec![];

    Screen::iter().for_each(|s| {
        let display_string = s.get_display_string();

        let style = if s == *app.current_screen() {
            Style::default().bg(Color::Blue).fg(Color::Black)
        } else {
            Style::default()
        };

        let new_item = ListItem::new(Line::from(Span::styled(display_string, style)));
        list_items.push(new_item);
    });

    let list = List::new(list_items);

    f.render_widget(list, inner_area);
}
