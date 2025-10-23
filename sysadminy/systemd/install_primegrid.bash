#!/bin/bash

set -eu

echo "Setting up systemd unit..."
systemd_unit="/etc/systemd/system/primegrid.service"
sudo touch "$systemd_unit"
sudo chmod 644 "$systemd_unit"

sudo bash -c "cat << EOF > "$systemd_unit"
[Unit]
Description=primegrid boinc worker
After=network.target

[Service]
Type=oneshot
User=chrome1020
ExecStart=/usr/bin/bash /home/chrome1020/primegrid.bash
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
EOF"

echo "Installed at $systemd_unit"

echo "Starting systemd unit"
set -x
sudo systemctl daemon-reload
sudo systemctl enable primegrid.service
