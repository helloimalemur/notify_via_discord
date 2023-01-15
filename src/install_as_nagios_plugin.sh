#!/bin/bash
rm -rf /tmp/notify_via_discord/ 2>/dev/null;
cd /tmp/;
git clone https://github.com/helloimalemur/notify_via_discord.git;
cd notify_via_discord;
cargo build;
if [[ -n /usr/local/nagios/libexec/ ]]; then echo "dir '/usr/local/nagios/libexec/' does not exist"; else sudo cp target/debug/notify_via_discord /usr/local/nagios/libexec/; fi;
