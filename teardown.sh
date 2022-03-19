#!/bin/bash
set -ex
sudo systemctl disable ipmon.service
sudo systemctl stop ipmon.service
sudo rm /etc/systemd/system/ipmon.service
sudo systemctl daemon-reload
