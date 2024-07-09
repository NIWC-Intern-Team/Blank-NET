use pnet::datalink::{self};
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::{icmp_packet_iter, transport_channel};
use pyo3::prelude::*;
use std::io::{Error, ErrorKind};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

// TODO create struct for serde deserial instead of manually typed
fn sniffer(
    function: Bound<PyAny>,
    interface: &String,
) -> std::io::Result<(i32, i32, Option<u32>, f64, String, Option<f64>)> {
    let func_call = function.call1((interface,))?;
    let json_str: &str = func_call.extract()?;
    let tup: (i32, i32, Option<u32>, f64, String, Option<f64>) = serde_json::from_str(json_str)?;
    Ok(tup)
}

pub fn scapy_analyzer_import() -> String {
    include_str!("../network_analyzer.py").into()
}

pub fn ping(ip: Ipv4Addr) -> bool {
    let protocol = Layer4(pnet::transport::TransportProtocol::Ipv4(
        IpNextHeaderProtocols::Icmp,
    ));
    let (mut tx, mut rx) =
        transport_channel(1024, protocol).expect("Error creating transport channel");

    // building ping packet
    let mut packet = [0u8; 16];
    let mut icmp_packet =
        MutableEchoRequestPacket::new(&mut packet).expect("Error creating echo packet");
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_sequence_number(1);
    icmp_packet.set_identifier(1);

    let checksum = pnet::util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);

    // try to ping 5 times before giving up
    for _ in 0..5 {
        tx.send_to(&icmp_packet, IpAddr::V4(ip))
            .expect("Error sending packet");

        let mut iter = icmp_packet_iter(&mut rx);
        match iter.next_with_timeout(Duration::from_millis(100)) {
            Ok(Some((_packet, addr))) => {
                if addr == std::net::IpAddr::V4(ip) {
                    return true;
                }
            }
            Ok(None) => {}
            Err(e) => {
                panic!("error: {}", e);
            }
        }
    }
    false
}

pub fn radio_metrics(
    code: &str,
    interface: &String,
) -> Result<(i32, i32, Option<u32>, f64, String, Option<f64>), Error> {
    // return Ok((0, 0, None, 0.0, "5ghz".into(), None));
    Python::with_gil(
        |py| -> Result<(i32, i32, Option<u32>, f64, String, Option<f64>), Error> {
            let scapy_analyzer =
                PyModule::from_code_bound(py, code, "network_analyzer.py", "network_analyzer")
                    .expect("hmm");
            let function = scapy_analyzer.getattr("radioSniffer").unwrap();
            match sniffer(function, interface) {
                Ok(res) => Ok(res),
                Err(_) => Err(Error::new(ErrorKind::Other, "Invalid json")),
            }
            // let res = sniffer(function)?;
            // let res = sniffer(function).unwrap();
            // Ok(res)
        },
    )
}

pub fn list_interfaces() -> Vec<String> {
    // grab interface names
    datalink::interfaces()
        .iter()
        .map(|ni| ni.name.clone())
        .collect()
}
