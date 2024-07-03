# GUSV-NET

A CLI tool for the GUSV to display and record network metrics. GUSV-NET assists GUSV test operators with features that allow analysis of network metrics and control of network configuration.


## Dependencies
This program required dependencies are listed below:
- Scapy
- libpcap
- rust / cargo

To install on ubuntu:

```
sudo apt-get update & upgrade
sudo apt-get install python3-scapy libpcap-dev

# also install rust and cargo if you haven't
```

> [!NOTE]
> In addition the binary has to be built from source with `cargo build`,


## Usage
run `sudo ./target/debug/gusvnet`


## Features

| Name | Status |
| -- | -- |
| Signal metric display | ✅ |
|  Save to PCAP file | ❌ |
| View network debug and control message | ❌ |
| Perform global network operations | ❌ |

