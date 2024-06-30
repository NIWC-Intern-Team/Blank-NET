use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::Terminal;
use std::error::Error;
use std::{io, u32};

mod app;
mod sniffer;
mod ui;
use crate::{app::*, ui::*};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
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
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exiting;
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
                            app.current_screen = CurrentScreen::Main;
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
        } else if app.current_screen == CurrentScreen::Main && app.options_idx == 1 {
            // TODO: I don't like this options index
            if let Ok(metrics) = sniffer::radio_metrics(&app.analyzer_code) {
                app.metrics = vec![
                    metrics.0.to_string(),
                    metrics.1.to_string(),
                    metrics.2.unwrap_or("N/A".to_string()),
                    metrics.3.to_string(),
                    metrics.4,
                    match metrics.5 {
                        None => "N/A".to_string(),
                        Some(a) => a.to_string(),
                    },
                ];
            }
        }
    }
}
