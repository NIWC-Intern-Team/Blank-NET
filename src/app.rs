use std::net::Ipv4Addr;

use crate::sniffer::*;

#[derive(PartialEq)]
pub enum CurrentScreen {
    Interface,
    Home,
    Main,
    Exiting,
}

#[derive(PartialEq)]
pub enum PingStatus {
    Halt,
    Running(u32),
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
    pub ip_group: Vec<[String; 2]>,
    pub ping_status: PingStatus
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
            ip_group: vec![
                ["192.168.1.201", "_"],
                ["192.168.1.202", "_"],
                ["192.168.1.200", "_"],
                ["192.168.1.110", "_"],
                ["192.168.1.112", "_"],
                ["192.168.1.113", "_"],
                ["192.168.1.114", "_"],
                ["192.168.1.115", "_"],
                ["129.168.1.116", "_"],
            ]
            .iter()
            .map(|s| [s[0].to_string(), s[1].to_string()])
            .collect(),
            interfaces: list_interfaces(),
            interface: String::new(),
            ping_status: PingStatus::Halt,
        }
    }
}
