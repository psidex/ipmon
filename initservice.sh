#!/bin/bash
set -x
sudo cp ipmon.service /etc/systemd/system
sudo systemctl daemon-reload
sudo systemctl enable ipmon.service
sudo systemctl start ipmon.service
sleep 3
sudo service ipmon status
