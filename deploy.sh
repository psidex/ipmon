#!/bin/bash
set -ex
go build -v -o ./ipmon ./cmd/ipmon
sudo cp ipmon.service /etc/systemd/system
sudo systemctl daemon-reload
sudo systemctl enable ipmon.service
sudo systemctl start ipmon.service
sleep 3
sudo service ipmon status
