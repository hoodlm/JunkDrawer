#! /bin/bash
set -eu

SESSION_NAME="simple-tmux-test"

# Simple example of starting two daemon processes in a background tmux session:
tmux new-session -s "$SESSION_NAME" -d "top"
tmux split-window -t "$SESSION_NAME" -d "ping 127.0.0.1"

echo "attaching in 3 seconds..."
sleep 3
tmux attach -t "$SESSION_NAME"
