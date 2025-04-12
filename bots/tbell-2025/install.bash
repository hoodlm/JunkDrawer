#!/bin/bash

set -eu

ruby_version="ruby-3.3.8"
runas_user="tbell-bot"

# Assumes a system (not single-user) installation of rvm is present.
echo "Installing system ruby ${ruby_version} if needed"
sudo /usr/share/rvm/bin/rvm install "$ruby_version"
echo "all system dependencies are present"

echo "Create runas_user $runas_user if needed..."
grep "$runas_user" /etc/passwd || sudo useradd --create-home --system "$runas_user"
sudo usermod -a -G rvm "$runas_user"

echo "Creating run script"
cat << EOF > run-tbell-bot.sh
#!/bin/bash

source /usr/share/rvm/scripts/rvm
rvm use ${ruby_version}
bundle
ruby /home/${runas_user}/tbell.rb
EOF

echo "Copying all files to $runas_user home directory"
sudo cp -v * "/home/${runas_user}/"
sudo chown -R ${runas_user} "/home/${runas_user}"

echo "Setting up systemd unit..."
systemd_unit="/etc/systemd/system/tbell-bot.service"
sudo touch "$systemd_unit"
sudo chmod 644 "$systemd_unit"

sudo bash -c "cat << EOF > "$systemd_unit"
[Unit]
Description=tbell telegram bot
After=network.target

[Service]
Type=simple
User=tbell-bot
ExecStart=/usr/bin/bash /home/tbell-bot/run-tbell-bot.sh
Restart=always

[Install]
WantedBy=multi-user.target
EOF"

echo "Starting systemd unit"
sudo systemctl daemon-reload
sudo systemctl enable tbell-bot.service
sudo systemctl start tbell-bot.service
sudo systemctl restart tbell-bot.service
