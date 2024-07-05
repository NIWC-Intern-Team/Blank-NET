use crate::app::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const SELECTED_STYLE: Style = Style::new().bg(Color::White).fg(Color::Black);

fn metric_block_ui(frame: &mut Frame, grid: Vec<Vec<Rect>>, app: &App) {
    let block_style = Style::default().fg(Color::Green);

    let metric_contents = [
        "Antenna",
        "Signal Strength",
        "Signal Noise",
        "Channel Freq",
        "Channel Flags",
        "Data Rate",
    ];
    for (idx, i) in metric_contents.iter().enumerate() {
        let metric_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
        let metric = Paragraph::new(Text::styled(*i, block_style))
            .block(metric_block)
            .alignment(Alignment::Center);
        frame.render_widget(metric, grid[idx / 3][idx % 3]);
        let metric_block = Block::default().style(Style::default());
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Length(1),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(grid[idx / 3][idx % 3]);
        let center = layout[1];
        let metric_val = Paragraph::new(Text::styled(app.metrics[idx].clone(), block_style))
            .block(metric_block)
            .alignment(Alignment::Center);
        frame.render_widget(metric_val, center);
    }
}

fn metric_ui(frame: &mut Frame, constraint: Vec<Rect>, app: &App) {
    let outer_metric_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(constraint[1]);

    let inner_metric_chunks_top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(outer_metric_chunks[0]);

    let inner_metric_chunks_bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(outer_metric_chunks[1]);

    metric_block_ui(
        frame,
        vec![
            inner_metric_chunks_top.to_vec(),
            inner_metric_chunks_bottom.to_vec(),
        ],
        app,
    );
}

fn select_ui(
    frame: &mut Frame,
    constraint: Vec<Rect>,
    app: &App,
    options: Vec<String>,
    options_idx: u32,
    have_border: bool,
) {
    let block_style = Style::default().fg(Color::White);
    let select_style = Style::default().fg(Color::Black);
    let constraints: Vec<Constraint> = options.iter().map(|_| Constraint::Fill(1)).collect();
    let option_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(constraint[1]);

    for (idx, i) in options.iter().enumerate() {
        let option_block = if !have_border {
            Block::default().style(Style::default())
        } else {
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
        };
        let option = if idx as u32 == options_idx {
            Paragraph::new(Text::styled(i, select_style))
                .block(if !have_border {
                    Block::default().style(Style::default().bg(Color::White).fg(Color::Black))
                } else {
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::White).fg(Color::Black))
                })
                .alignment(Alignment::Left)
        } else {
            Paragraph::new(Text::styled(i, block_style))
                .block(option_block)
                .alignment(Alignment::Left)
        };
        frame.render_widget(option, option_chunks[idx]);
    }
}

fn home_ui(frame: &mut Frame, constraints: Vec<Rect>, app: &App) {
    select_ui(
        frame,
        constraints,
        app,
        app.options.to_vec(),
        app.options_idx,
        true,
    )
}

fn connection_ui(frame: &mut Frame, constraint: Vec<Rect>, app: &mut App) {
    if app.current_screen == CurrentScreen::NodeView(Mode::Editing) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(5)])
            .split(constraint[1]);

        app.node_table.render(chunks[0], frame);

        let border_color = if app.ip_input_status == IpInputStatus::Error {
            Color::Red
        } else {
            Color::White
        };
        let block = Block::default()
            .title("New IP:")
            .borders(Borders::ALL)
            .fg(border_color);

        // .border_style(block_border_style);
        let text = Paragraph::new(app.ip_input.clone())
            .block(block)
            .fg(Color::White);
        frame.render_widget(text, chunks[1]);
    } else {
        app.node_table.render(constraint[1], frame);
    }
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    match app.current_screen {
        CurrentScreen::NodeView(_) => connection_ui(f, chunks.to_vec(), app),
        CurrentScreen::Main => match app.options_idx {
            1 => metric_ui(f, chunks.to_vec(), app),
            _ => {}
        },
        CurrentScreen::InterfaceView => {
            let new_if_list = app.interfaces.clone();
            select_ui(
                f,
                chunks.to_vec(),
                app,
                new_if_list,
                app.if_options_idx,
                false,
            );
        }
        CurrentScreen::Home | CurrentScreen::Exiting => home_ui(f, chunks.to_vec(), app),
    }

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title: Paragraph =
        Paragraph::new(Text::styled("GUSV-NET", Style::default().fg(Color::Green)))
            .block(title_block);

    let mut current_navigation_text = if app.current_screen == CurrentScreen::Exiting {
        vec![Span::styled(
            "[press q again to exit]  ",
            Style::default().fg(Color::LightRed),
        )]
    } else {
        vec![Span::styled(
            "[q - quit]  ",
            Style::default().fg(Color::LightYellow),
        )]
    };

    // TODO: match for other screen types
    match app.current_screen {
        CurrentScreen::NodeView(_) => current_navigation_text.append(&mut vec![
            Span::styled(
                "[p - start ping test]  ",
                Style::default().fg(Color::LightYellow),
            ),
            Span::styled(
                "[enter - to edit node]  ",
                Style::default().fg(Color::LightYellow),
            ),
        ]),
        _ => {}
    };

    let navigation_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    // -- All rendering should happen below! --
    f.render_widget(title, chunks[0]);
    f.render_widget(navigation_footer, chunks[2]);
}
