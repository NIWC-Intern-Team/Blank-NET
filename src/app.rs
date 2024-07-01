use crate::sniffer::*;

#[derive(PartialEq)]
pub enum CurrentScreen {
    Interface,
    Home,
    Main,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub metrics: Vec<String>,
    pub analyzer_code: String,
    pub options_idx: u32,
    pub options: [String; 3],
    pub interface: String,
    pub if_options_idx: u32,
    pub interfaces: Vec<String>,
}

impl App {
    pub fn new() -> App {
        App {
            // current_screen: CurrentScreen::Main,
            current_screen: CurrentScreen::Home,
            metrics: (0..6).map(|_| String::new()).collect(),
            analyzer_code: scapy_analyzer_import(),
            options_idx: 0,
            if_options_idx: 0,
            options: [
                "Connection test".into(),
                "GUSV network metrics".into(),
                "Something else".into(),
            ],
            interfaces: list_interfaces(),
            interface: String::new(),
        }
    }
}
