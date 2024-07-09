use ratatui::{
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::App;

const HELPTEXT: &'static str = "
    GUSVNET is commandline tool for the GUSV to perform multi-pings and display network metrics.
    There are two main features:

    1. Connection test:

        Performs a ping test for a list of IP addresses (nodes). This option is fully editable
        either through GUSVNET or in JSON array format. To edit the node simply press enter on
        the selected target node. You can then type a new IP address

    2. Network monitor:
    
        A display that shows radio tap metrics such as signal strength, signal monitor, etc.
";

pub fn render(frame: &mut Frame, constraint: Vec<Rect>, app: &mut App) {
    let block = Block::default().borders(Borders::ALL).fg(Color::White);
    let text = Paragraph::new(HELPTEXT)
        .block(block)
        .scroll((app.help_scroll, 0));
    frame.render_widget(text, constraint[0]);
}
