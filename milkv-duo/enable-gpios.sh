#!/bin/sh
# configure only Header J3
# https://milkv.io/docs/duo/getting-started/duos#header-j3
IN_1=468
IN_2=469
IN_3=466
IN_4=459

if [ $# -eq 0 ]; then
    echo "Usage: $0 <1|2|3|4>"
    exit 1
fi

duo-pinmux -w B20/B20
duo-pinmux -w B21/B21
duo-pinmux -w B18/B18
duo-pinmux -w B11/B11

case "$1" in
    1) switch_pin=$IN_1 ;;
    2) switch_pin=$IN_2 ;;
    3) switch_pin=$IN_3 ;;
    4) switch_pin=$IN_4 ;;
    *) echo "Invalid option: $1. Please use 1, 2, 3, or 4." ; exit 1 ;;
esac

GPIO=/sys/class/gpio/gpio${switch_pin}

if test -d ${GPIO}; then
    echo "PIN ${switch_pin} already exported"
else
    echo ${switch_pin} > /sys/class/gpio/export
fi

echo out > ${GPIO}/direction

while true; do
    echo 0 > ${GPIO}/value
    sleep 1
    echo 1 > ${GPIO}/value
    sleep 1
done
