use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::Terminal;
use serde_json::json;
use sniffer::ping;
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::net::Ipv4Addr;
use std::path::PathBuf;

mod app;
mod helpview;
mod nodetable;
mod scrollview;
mod sniffer;
mod ui;

use crate::{app::*, nodetable::*, ui::*};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./blanknet.yaml")]
    config_file: PathBuf,
}

fn config_import(filepath: &PathBuf) -> (usize, String) {
    let mut file = if let Ok(tmp_file) = fs::File::create_new(filepath) {
        tmp_file
    } else {
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(filepath)
            .expect("Cannot open file")
    };
    let mut buf = String::new();
    let size = file.read_to_string(&mut buf).expect("Failed to read file");
    (size, buf)
}

fn save_config(app: &mut App) {
    let mut file = fs::File::create(&app.filepath).unwrap();
    let list: Vec<String> = app
        .node_table
        .nodes
        .iter()
        .map(|node| node.ip.clone())
        .collect();
    let json_str = json!(list).to_string();
    file.write_all(json_str.as_bytes()).unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut app = App::new();

    app.filepath = args.config_file;
    let (size, buf) = config_import(&app.filepath);
    if size != 0 {
        println!("{}", buf);
        let ip_list: Vec<String> = serde_json::from_str(&buf).unwrap();
        for ip in &ip_list {
            if ip.parse::<Ipv4Addr>().is_err() {
                panic!("Format error: Expected array of IPV4 Addresses");
            }
        }
        app.node_table = ip_list
            .iter()
            .map(|ip| (ip.clone(), "-".to_string()))
            .collect();
    }

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
                    CurrentScreen::NodeView(Mode::Delete) => match key.code {
                        KeyCode::Char('y') => {
                            app.node_table
                                .delete_ip(app.node_table.get_selected_idx() as u32);
                            app.current_screen = CurrentScreen::NodeView(Mode::Normal);
                        }
                        KeyCode::Char('n') => {
                            app.current_screen = CurrentScreen::NodeView(Mode::Normal)
                        }
                        _ => {}
                    },
                    CurrentScreen::NodeView(Mode::Edit) | CurrentScreen::NodeView(Mode::Push) => {
                        match key.code {
                            KeyCode::Enter => {
                                if app.ip_input.parse::<Ipv4Addr>().is_ok() {
                                    if app.current_screen == CurrentScreen::NodeView(Mode::Push) {
                                        app.node_table.create_ip(app.ip_input.clone());
                                    } else {
                                        app.node_table.update_ip(app.ip_input.clone());
                                    }
                                    save_config(app);
                                    app.ip_input_status = IpInputStatus::Normal;
                                    app.current_screen = CurrentScreen::NodeView(Mode::Normal)
                                } else if app.ip_input.is_empty() {
                                    app.ip_input_status = IpInputStatus::Normal;
                                    app.current_screen = CurrentScreen::NodeView(Mode::Normal)
                                } else {
                                    app.ip_input_status = IpInputStatus::Error;
                                }
                            }

                            KeyCode::Backspace => {
                                app.ip_input.pop();
                            }
                            KeyCode::Char(val) => {
                                app.ip_input.push(val);
                            }
                            _ => {}
                        }
                    }
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
                        //TODO: rename to create instead of push
                        KeyCode::Char('c') => {
                            app.current_screen = CurrentScreen::NodeView(Mode::Push);
                        }
                        KeyCode::Char('d') => {
                            app.current_screen = CurrentScreen::NodeView(Mode::Delete);
                        }
                        KeyCode::Enter => {
                            // app.current_screen =
                            //     if app.current_screen == CurrentScreen::NodeView(Mode::Edit) {
                            //         CurrentScreen::NodeView(Mode::Normal)
                            //     } else {
                            app.ip_input = app.node_table.get_selected_ip();
                            app.current_screen = CurrentScreen::NodeView(Mode::Edit)
                            // }
                        }
                        _ => {}
                    },
                    CurrentScreen::Help => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Home;
                            app.ping_status = PingStatus::Halt;
                            app.metrics = (0..6).map(|_| String::new()).collect();
                        }
                        KeyCode::Down => {
                            app.help_scroll = if app.help_scroll != 20 {
                                app.help_scroll + 1
                            } else {
                                20
                            };
                        }
                        KeyCode::Up => {
                            app.help_scroll = if app.help_scroll != 0 {
                                app.help_scroll - 1
                            } else {
                                0
                            };
                        }
                        _ => {}
                    },
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Home;
                            app.ping_status = PingStatus::Halt;
                            app.metrics = (0..6).map(|_| String::new()).collect();
                        }
                        // KeyCode::Enter => {
                        //     if app.options_idx == 0 {
                        //         app.ping_status = PingStatus::Running(0);
                        //     }
                        // }
                        _ => {}
                    },
                    CurrentScreen::InterfaceView => match key.code {
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
                                app.current_screen = CurrentScreen::InterfaceView;
                            } else if app.options_idx == 2 {
                                app.current_screen = CurrentScreen::Help;
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
                        _ => app.current_screen = CurrentScreen::Home,
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
                CurrentScreen::Main => {
                    if app.interface.is_empty() {
                        continue;
                    }
                    if let Ok(metrics) = sniffer::radio_metrics(&app.analyzer_code, &app.interface)
                    {
                        app.metrics = vec![
                            metrics.0.to_string(),
                            metrics.1.to_string(),
                            metrics.2.unwrap_or(0).to_string(),
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
