#!/bin/bash
set -eu

# REMEMBER TO SET ACCOUNT KEY! (<ACCOUNTKEYHERE>)
export PATH="/usr/bin/:/usr/local/bin/:/snap/bin/:/usr/local/sbin:/usr/sbin:/sbin"

apt install -y tmux libxss1 x11-common
snap install aws-cli --classic
# Needed for IPv6 S3 support
aws configure set default.s3.use_dualstack_endpoint true
aws --region "us-east-2" s3 cp "s3://103614357797-software-bucket/boinc-client_8.2.5-2581_amd64_noble.deb" .
dpkg --install boinc-client*.deb

sudo -u ubuntu -i <<'EOF'
mkdir -pv /home/ubuntu/primegrid"
cd /home/ubuntu/primegrid"


tmux new-session -s "primegrid" -d "boinc --gui_rpc_port 37777"
tmux split-window -t "primegrid" -d \
    "sleep 5 \
    && boinccmd --host 127.0.0.1:37777 --project_attach https://www.primegrid.com <ACCOUNTKEYHERE> \
    && watch -n3 boinccmd --host 127.0.0.1:37777 --get_task_summary"
EOF
