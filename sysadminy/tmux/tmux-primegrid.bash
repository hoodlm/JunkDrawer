#! /bin/bash

if test -z $ACCOUNT_KEY; then
    echo "Must set env variable \$ACCOUNT_KEY" > /dev/stderr
    exit 1
fi
set -eu


SESSION_NAME="primegrid"

WORKING_DIR="$HOME/primegrid"
mkdir -pv $WORKING_DIR
pushd $WORKING_DIR

tmux new-session -s "$SESSION_NAME" -d "boinc --gui_rpc_port 37777"
tmux split-window -t "$SESSION_NAME" -d \
    "sleep 5 \
    && boinccmd --host 127.0.0.1:37777 --project_attach https://www.primegrid.com $ACCOUNT_KEY \
    && watch -n10 boinccmd --host 127.0.0.1:37777 --get_task_summary ; $SHELL"

# Open a third pane and leave this as the active pane if the user attaches:
tmux select-pane -D -t "$SESSION_NAME"

echo "primegrid running in background tmux session $SESSION_NAME"
echo "to attach:"
echo "tmux attach -t $SESSION_NAME"
