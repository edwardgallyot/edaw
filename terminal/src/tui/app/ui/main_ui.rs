use super::{super::screens::Screen, App};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, List, ListItem},
    Frame,
};
use strum::IntoEnumIterator;

const INNER_OFFSET: u16 = 3;

pub fn draw_main_ui(f: &mut Frame, app: &App) {
    let title = Title::from("Edaw Terminal").alignment(ratatui::layout::Alignment::Center);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default());
    
    f.render_widget(title_block, f.size());

    let mut inner_area = f.size();

    inner_area.width -= INNER_OFFSET;
    inner_area.height -= INNER_OFFSET;

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

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);


    for chunk in chunks.iter() {
        f.render_widget(list, *chunk);
    }

}
