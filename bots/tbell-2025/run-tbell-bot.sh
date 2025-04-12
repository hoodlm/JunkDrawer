#!/bin/bash

source /usr/share/rvm/scripts/rvm
rvm use ruby-3.3.8
bundle
ruby /home/tbell-bot/tbell.rb
