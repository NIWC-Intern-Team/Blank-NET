# BLANK-NET

A CLI tool for the BLANK to display and record network metrics. BLANK-NET assists BLANK test operators with features that allow analysis of network metrics and control of network configuration.

### Dependencies
This program required dependencies are listed below:
- rust / cargo `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Scapy
- libpcap
## I don't want to install each dependency just let me copy and run!
Alright here you go:
```bash
sudo apt-get update & upgrade &&
apt-get install python3-scapy libpcap-dev curl &&
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
&& pip install scapy
```

  
> [!NOTE]
> In addition the binary has to be built from source with `cargo build` or `cargo build --release` The binaries will be stores in the respective paths: `./target/debug/gusvnet`, `./target/release/gusvnet`
  
### Usage

run `sudo ./target/debug/gusvnet` or `sudo ./target/release/gusvnet`
### Features
- Verify connection of multiple IP addresses using ping
- IP address Configuration
-  Auto saves edits to nodes (This might not be fully complete)
- Live network signal monitoring

### TODOS:
- [ ] Show consistent round-trip time alongside connection status if connected
- [ ] Improve security of the application
- [ ] Introduce unit tests, refer to Ratatui demos
- [ ] Clean up overall UI
