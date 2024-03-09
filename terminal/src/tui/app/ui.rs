use ratatui::{Frame, widgets::{Block, Borders, block::Title, ListItem, List}, style::{Style, Color}, text::{Line, Span}, layout::{Layout, Direction, Constraint}};
use strum::IntoEnumIterator;

use super::{App, screens::Screen};


pub fn ui(f: &mut Frame, app: &App) {

    let title = Title::from("Edaw Terminal").alignment(ratatui::layout::Alignment::Center);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default());


    let mut constraints = vec![];
    for _ in Screen::iter() {
        constraints.push(
            Constraint::Length(4)
        )
    }

    let mut list_items = vec![];

    Screen::iter().for_each(|s| {
        let display_string = s.get_display_string();

        let style = if s == *app.current_screen() {
            Style::default().bg(Color::Blue).fg(Color::Black)
        } else {
            Style::default()
        };

        let new_item = ListItem::new(Line::from(Span::styled(
            display_string,
            style,
        )));

        list_items.push(new_item);
    });

    let list = List::new(list_items);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.size());

    f.render_widget(title_block, chunks[0]);
    f.render_widget(list, chunks[1]);
}
