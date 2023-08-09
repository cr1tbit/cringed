#!/bin/bash

# for development purposes only - launching this script
# will explode your PC btw so don't do it, who marked
# this as executable anyway?

CURR_DIR=$(pwd)

# ln -s $CURR_DIR/99-cringedge.rules /etc/udev/rules.d/.
# ln -s $CURR_DIR/cringedge@.service /etc/systemd/system/.
# ln -s $CURR_DIR/mockstarter.sh /opt/cringed/spawn_cringedge


# assert if cringed binary exists
if [ ! -f /opt/cringed/cringed ]; then
    echo "cringed binary not found, exiting"
    exit 1
fi

