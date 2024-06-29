use crate::sniffer::*;

pub enum CurrentScreen {
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
            current_screen: CurrentScreen::Main,
            metrics: (0..6).into_iter().map(|_| String::new()).collect(),
            analyzer_code: scapy_analyzer_import(),
        }
    }
}
