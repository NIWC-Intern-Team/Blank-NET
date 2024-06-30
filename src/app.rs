use crate::sniffer::*;

#[derive(PartialEq)]
pub enum CurrentScreen {
    Home,
    Main,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub metrics: Vec<String>,
    pub analyzer_code: String,
}

impl App {
    pub fn new() -> App {
        App {
            // current_screen: CurrentScreen::Main,
            current_screen: CurrentScreen::Home,
            metrics: (0..6).map(|_| String::new()).collect(),
            analyzer_code: scapy_analyzer_import(),
        }
    }
}
