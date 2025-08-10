#!/bin/bash
set -eux -o pipefail

mkdir -pv ./data
pushd data && wget https://www.voynich.nu/hist/reeds/FSG.txt
