use ratatui::{prelude::*, widgets::*};

pub fn render(area: Rect, frame: &mut Frame, state: &mut ScrollbarState) {
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    frame.render_stateful_widget(scrollbar, area, state);
}
