#!/bin/sh
# configure only Header J3
# https://milkv.io/docs/duo/getting-started/duos#header-j3
duo-pinmux -w B20 B20
ceiling_heating_pump=451

GPIO=/sys/class/gpio/gpio${ceiling_heating_pump}

if test -d ${GPIO}; then
    echo "PIN ${ceiling_heating_pump} already exported"
else
    echo ${ceiling_heating_pump} > /sys/class/gpio/export
fi

echo out > ${GPIO}/direction

while true; do
    echo 0 > ${GPIO}/value
    sleep 1
    echo 1 > ${GPIO}/value
    sleep 1
done
