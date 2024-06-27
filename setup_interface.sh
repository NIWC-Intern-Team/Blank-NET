#!/bin/bash
# change for your interface

sudo ifconfig wlx8c3badbc5be7l down

sudo iwconfig wlx8c3badbc5be7 mode monitor

sudo ifconfig wlx8c3badbc5be7l up
