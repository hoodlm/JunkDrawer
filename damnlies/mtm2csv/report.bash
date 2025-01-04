#! /usr/bin/env bash

set -eu -o pipefail

sqlite3 -batch < report.sql
