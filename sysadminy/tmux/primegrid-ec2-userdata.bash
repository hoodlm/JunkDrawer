#! /bin/bash
set -eu

# REMEMBER TO SET ACCOUNT KEY! (<ACCOUNTKEYHERE>)

export PATH="/usr/bin/:/usr/local/bin/"

set +eu
if test -z $ACCOUNT_KEY; then
    echo "Must set env variable \$ACCOUNT_KEY" > /dev/stderr
    exit 1
fi
set -eu

curl -fsSL https://boinc.berkeley.edu/dl/linux/stable/noble/boinc.gpg | sudo gpg --dearmor -o /etc/apt/keyrings/boinc.gpg
echo deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/boinc.gpg] \
    https://boinc.berkeley.edu/dl/linux/stable/noble noble main \
    | tee /etc/apt/sources.list.d/boinc.list > /dev/null
apt update
apt install -y boinc-client tmux

sudo -u ubuntu -i <<'EOF'
mkdir -pv /home/ubuntu/primegrid
cd /home/ubuntu/primegrid

tmux new-session -s "primegrid" -d "boinc --gui_rpc_port 37777"
tmux split-window -t "primegrid" -d \
    "sleep 5 \
    && boinccmd --host 127.0.0.1:37777 --project_attach https://www.primegrid.com <ACCOUNTKEYHERE> \
    && watch -n10 boinccmd --host 127.0.0.1:37777 --get_task_summary ; $SHELL"

tmux select-pane -D -t "primegrid"
EOF
