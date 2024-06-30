use crate::app::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

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
                    Constraint::Length(5),
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

fn home_ui(frame: &mut Frame, constraints: Vec<Rect>, app: &App) {
    let block_style = Style::default().fg(Color::White);

    let option_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Ratio(1, app.options.len() as u32),
            Constraint::Ratio(1, app.options.len() as u32),
            Constraint::Ratio(1, app.options.len() as u32),
        ])
        .split(constraints[1]);

    for (idx, i) in app.options.iter().enumerate() {
        let option_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
        let option;
        if idx as u32 == app.options_idx {
            option = Paragraph::new(Text::styled(i, block_style))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::White).fg(Color::Black)),
                )
                .alignment(Alignment::Left);
        } else {
            option = Paragraph::new(Text::styled(i, block_style))
                .block(option_block)
                .alignment(Alignment::Left);
        }
        frame.render_widget(option, option_chunks[idx]);
    }
}

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    match app.current_screen {
        CurrentScreen::Main => {
            // TODO: Can we have scroll over enum? Would be more idiomatic?
            match app.options_idx {
                1 => metric_ui(f, chunks.to_vec(), app),
                _ => {}
            }
        }
        CurrentScreen::Home => home_ui(f, chunks.to_vec(), app),
        _ => {}
    }
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("GUSV-NET", Style::default().fg(Color::Green)))
        .block(title_block);

    let current_navigation_text = match app.current_screen {
        CurrentScreen::Main | CurrentScreen::Home => {
            vec![
                Span::styled("q - quit", Style::default().fg(Color::LightYellow)),
                // TODO:  Add in once navigation is handled
                //
                // Span::styled(
                //     " navigation - arrow keys",
                //     Style::default().fg(Color::LightYellow),
                // ),
            ]
        }

        CurrentScreen::Exiting => vec![Span::styled(
            "press q again to exit",
            Style::default().fg(Color::LightRed),
        )],
    };

    let navigation_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    // -- All rendering should happen below! --
    f.render_widget(title, chunks[0]);
    match app.current_screen {
        CurrentScreen::Home => {}
        _ => {}
    }
    f.render_widget(navigation_footer, chunks[2]);
}
