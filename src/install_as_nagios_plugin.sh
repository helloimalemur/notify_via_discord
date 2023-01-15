#!/bin/bash
cd /tmp/
git clone https://github.com/helloimalemur/notify_via_discord.git
cd notify_via_discord
cargo build
sudo cp target/debug/notify_via_discord /usr/local/nagios/libexec/
