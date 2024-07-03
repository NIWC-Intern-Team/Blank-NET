use ratatui::{prelude::*, widgets::*};

use crate::scrollview;

const HEADER_STYLE: Style = Style::new().fg(Color::Green);
const SELECTED_STYLE: Style = Style::new().fg(Color::White).bg(Color::Green); // TODO: Pick nice select style
const DATA_STYLE: Style = Style::new().fg(Color::White);

const ITEM_HEIGHT: usize = 2;

pub struct Node {
    pub ip: String,
    pub conn_status: String,
    // TODO: Add more here
}

impl From<&Node> for [String; 2] {
    fn from(val: &Node) -> Self {
        [val.ip.clone(), val.conn_status.clone()]
    }
}

impl Node {
    pub fn from_tuple(tuple: (String, String)) -> Self {
        Self {
            ip: tuple.0,
            conn_status: tuple.1,
        }
    }
}

pub struct NodeTable {
    pub fields: Vec<String>,
    pub nodes: Vec<Node>,
    state: TableState,
    scroll_state: ScrollbarState,
}

impl FromIterator<(String, String)> for NodeTable {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let nodes: Vec<Node> = iter.into_iter().map(Node::from_tuple).collect();
        let length = nodes.len();
        Self {
            nodes,
            fields: vec!["IP".into(), "Connection status".into()],
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((length - 1) * ITEM_HEIGHT),
        }
    }
}

impl NodeTable {
    pub fn new(fields: Vec<String>, nodes: Vec<Node>) -> Self {
        let length = nodes.len();
        Self {
            fields,
            nodes,
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((length - 1) * ITEM_HEIGHT),
        }
    }

    pub fn default() -> Self {
        vec![
            ("192.168.1.201", "_"),
            ("192.168.1.202", "_"),
            ("192.168.1.200", "_"),
            ("192.168.1.110", "_"),
            ("192.168.1.112", "_"),
            ("192.168.1.113", "_"),
            ("192.168.1.114", "_"),
            ("192.168.1.115", "_"),
            ("129.168.1.116", "_"),
        ]
        .iter()
        .map(|f| (f.0.to_string(), f.1.to_string()))
        .collect()
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.nodes.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn update_ip(&mut self, ip: String) {
        self.nodes[self.state.selected().unwrap()].ip = ip;
    }

    pub fn get_selected_ip(&mut self) -> String {
        self.nodes[self.state.selected().unwrap()].ip.clone()
    }

    pub fn update_conn_status(&mut self, idx: usize, status: String) {
        self.nodes[idx].conn_status = status;
    }

    pub fn reset_conn_status(&mut self) {
        for i in &mut self.nodes {
            i.conn_status = "-".into();
        }
    }

    pub fn render(&mut self, area: Rect, frame: &mut Frame) {
        let rows: Vec<Row> = self
            .nodes
            .iter()
            .map(|f| {
                Row::new::<[String; 2]>(f.into())
                    .style(DATA_STYLE)
                    .height(2)
            })
            .collect();

        // Columns widths are constrained in the same way as Layout...
        let widths: Vec<Constraint> = self
            .fields
            .iter()
            .map(|_| Constraint::Percentage(30))
            .collect();
        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(Style::new().blue())
            .header(
                Row::new(self.fields.clone())
                    .style(HEADER_STYLE)
                    .bold()
                    .height(1)
                    .bottom_margin(1),
            )
            // .footer(Row::new(vec!["Updated on Dec 28"])) TODO(Feature): Correct data of edit
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>");

        scrollview::render(
            area.inner(Margin {
                vertical: 1,
                horizontal: 4,
            }),
            frame,
            &mut self.scroll_state,
        );
        frame.render_stateful_widget(table, area, &mut self.state);
    }
}
