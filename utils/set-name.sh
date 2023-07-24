#!/bin/sh

#not used for now, just playground

if [[ -z "${ID_SERIAL_SHORT}" ]]; then
  exit 1
else
  # NAME=$(echo $ID_SERIAL_SHORT | cut -d: -f4- | sed 's/:/_/g')
  /usr/bin/systemd-escape -p --template=test-@.service $NAME
  echo $NAME
  exit 0
fi

#SUBSYSTEM=="tty", ATTRS{idVendor}=="303a", ATTRS{idProduct}=="1001", PROGRAM+="/are/outduino/outduino-fw/utils/set-name.sh" SYMLINK+="outduino_%c"%

#printenv > /tmp/log
