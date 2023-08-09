#!/bin/bash

# for development purposes only - launching this script
# will explode your PC btw so don't do it, who marked
# this as executable anyway?
cd ..
CURR_DIR=$(pwd)

# ln -s $CURR_DIR/99-cringedge.rules /etc/udev/rules.d/.
# ln -s $CURR_DIR/cringedge@.service /etc/systemd/system/.
# ln -s $CURR_DIR/mockstarter.sh /opt/cringed/spawn_cringedge

# cargo build

mkdir -p /opt/cringed

ln -s $CURR_DIR/target/debug/daemon /opt/cringed/cringed-daemon
ln -s $CURR_DIR/target/debug/client /opt/cringed/cringed
ln -s $CURR_DIR/utils/cringed.service /etc/systemd/system/.

ln -s /opt/cringed/cringed /usr/bin/cringed

systemctl enable cringed.service


