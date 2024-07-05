#!/bin/env python3

import sys
import subprocess
from scapy.all import sr1, IP, ICMP, sniff, Dot11, RadioTap
import logging
import argparse
import json

result = ''

'''
@return:
    antenna,
    antenna signal / rssi,
    antenna noise,
    chan frequency,
    chan flags,
    rate,
'''
def signalMetrics(radiotap):
    global result
    result = (
            radiotap.Antenna,
            radiotap.dBm_AntSignal,
            radiotap.dBm_AntNoise,
            radiotap.ChannelFrequency,
            str(radiotap.ChannelFlags),
            radiotap.Rate
            )

def processRadioTap(packet):

    # 802.11 packet
    if packet.haslayer(RadioTap):
        radiotap = packet.getlayer(RadioTap)
        signalMetrics(radiotap)


def radioSniffer(interface):
    sniff(iface=interface, prn=lambda packet: processRadioTap(packet), store = 0, count = 1, timeout = 0.5)
    return json.dumps(result)

if __name__ == '__main__':

    # logging.getLogger("scapy").setLevel(logging.CRITICAL)
    parser = argparse.ArgumentParser(prog="Interface sniffer")
    parser.add_argument('interface')
    args = parser.parse_args()
    while True:
        print(radioSniffer(args.interface))



