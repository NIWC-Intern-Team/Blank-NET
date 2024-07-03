use ratatui::{prelude::*, widgets::*};

pub fn render(area: Rect, frame: &mut Frame, state: &mut ScrollbarState) {
    // 100 lines of text
    // TODO: feed in tables
    let line_numbers = (1..=100).map(|i| format!("{:>3} ", i)).collect::<String>();
    let content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n".repeat(100);


    let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    // the layout doesn't have to be hardcoded like this, this is just an example

    // scroll_view.render_widget(Paragraph::new(line_numbers), Rect::new(0, 0, 5, 100));
    // scroll_view.render_widget(Paragraph::new(content), Rect::new(5, 0, 95, 100));
    frame.render_stateful_widget(scrollbar, area, state);
}
