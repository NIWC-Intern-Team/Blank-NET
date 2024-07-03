use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::Terminal;
use sniffer::ping;
use std::error::Error;
use std::io;

mod app;
mod nodetable;
mod scrollview;
mod sniffer;
mod ui;
use crate::{app::*, nodetable::*, ui::*};

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let _res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match app.current_screen {
                    CurrentScreen::NodeView(_) => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Home;
                            app.ping_status = PingStatus::Halt;
                        }
                        KeyCode::Char('p') => {
                            app.node_table.reset_conn_status();
                            app.ping_status = PingStatus::Running(0);
                        }
                        KeyCode::Down => {
                            if app.current_screen == CurrentScreen::NodeView(Mode::Normal) {
                                app.node_table.next();
                            }
                        }
                        KeyCode::Up => {
                            if app.current_screen == CurrentScreen::NodeView(Mode::Normal) {
                                app.node_table.previous();
                            }
                        }
                        KeyCode::Enter => {
                            app.current_screen =
                                if app.current_screen == CurrentScreen::NodeView(Mode::Editing) {
                                    CurrentScreen::NodeView(Mode::Normal)
                                } else {
                                    CurrentScreen::NodeView(Mode::Editing)
                                }
                        }
                        _ => {}
                    },
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Home;
                            app.ping_status = PingStatus::Halt;
                            app.metrics = (0..6).map(|_| String::new()).collect();
                        }
                        KeyCode::Enter => {
                            if app.options_idx == 0 {
                                app.ping_status = PingStatus::Running(0);
                            }
                        }
                        _ => {}
                    },
                    CurrentScreen::Interface => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Home;
                        }
                        KeyCode::Enter => {
                            app.current_screen = CurrentScreen::Main;
                            app.interface
                                .clone_from(&app.interfaces[app.if_options_idx as usize]);
                        }
                        KeyCode::Up => {
                            app.if_options_idx = if app.if_options_idx == 0 {
                                (app.interfaces.len() - 1) as u32
                            } else {
                                app.if_options_idx - 1
                            }
                        }
                        KeyCode::Down => {
                            app.if_options_idx =
                                (app.if_options_idx + 1) % app.interfaces.len() as u32;
                        }
                        _ => {}
                    },
                    CurrentScreen::Home => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exiting;
                        }
                        KeyCode::Up => {
                            app.options_idx = if app.options_idx == 0 {
                                (app.options.len() - 1) as u32
                            } else {
                                app.options_idx - 1
                            }
                        }
                        KeyCode::Down => {
                            app.options_idx = (app.options_idx + 1) % app.options.len() as u32;
                        }
                        KeyCode::Enter => {
                            if app.options_idx == 0 {
                                app.current_screen = CurrentScreen::NodeView(Mode::Normal);
                            } else if app.options_idx == 1 {
                                app.current_screen = CurrentScreen::Interface;
                            } else {
                                app.current_screen = CurrentScreen::Main;
                            }
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('q') => {
                            return Ok(true);
                        }
                        _ => app.current_screen = CurrentScreen::Main,
                    },
                }
            }
        } else {
            match app.current_screen {
                CurrentScreen::NodeView(_) => {
                    if let PingStatus::Running(idx) = app.ping_status {
                        if idx == app.node_table.nodes.len() as u32 {
                            app.ping_status = PingStatus::Halt
                        } else {
                            // TODO: This is crap, fix clones
                            // perform ping at idx
                            app.node_table.update_conn_status(
                                idx as usize,
                                if ping(app.node_table.nodes[idx as usize].ip.parse().unwrap()) {
                                    "connected".to_string()
                                } else {
                                    "disconnected".to_string()
                                },
                            );

                            app.ping_status = PingStatus::Running(idx + 1)
                        }
                    }
                }
                CurrentScreen::Interface => {
                    // TODO: I don't like this options index
                    if let Ok(metrics) = sniffer::radio_metrics(&app.analyzer_code, &app.interface)
                    {
                        app.metrics = vec![
                            metrics.0.to_string(),
                            metrics.1.to_string(),
                            metrics.2.unwrap_or(0).to_string(),
                            // metrics.2.unwrap_or("N/A".to_string()),
                            metrics.3.to_string(),
                            metrics.4,
                            match metrics.5 {
                                None => "N/A".to_string(),
                                Some(a) => a.to_string(),
                            },
                        ];
                    }
                }
                _ => {}
            }
        }
    }
}
