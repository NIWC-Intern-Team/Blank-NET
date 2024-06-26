#!/bin/env python3

import sys
import subprocess
from scapy.all import sr1, IP, ICMP, sniff, Dot11, RadioTap
import logging
import argparse

def signalMetrics(radiotap):
    if (radiotap.dBm_AntSignal):
        rssi = radiotap.dBm_AntSignal
        print(f"RSSI={rssi}")
        if (radiotap.dBm_AntNoise):
            snr = rssi / radiotap.dBm_AntNoise
            print(f"SNR={snr}")

def processRadioTap(packet):

    # 802.11 packet
    if packet.haslayer(RadioTap):
        radiotap = packet.getlayer(RadioTap)
        signalMetrics(radiotap)
    #else:
    #    print(packet.summary())


if __name__ == '__main__':
    # logging.getLogger("scapy").setLevel(logging.CRITICAL)

    parser = argparse.ArgumentParser(prog="Interface sniffer")
    parser.add_argument('-i', '--interactive', action='store_true')
    parser.add_argument('interface')
    args = parser.parse_args()
    
    if args.interactive:
        print('GUSV-NET: Network CLI')
        print('Options:')
        print('1) Network metrics')
        print('2) Capture pcap files')
        option = input('Choose your option: ')
        if option == '1':
            interface = input("Choose your interface: ")
            sniff(iface=interface, prn=lambda packet: processRadioTap(packet), store = 0)
    else:
        sniff(iface=args.interface, prn=lambda packet: processRadioTap(packet), store = 0)

