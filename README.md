# Notify_Via_Discord
    Rust micro-app used to send messages to Discord web-hook ~(Nagios compatible)~

    sends POST to specified Discord webhook url using specified username and message

    Format: ./notify_discord $api_url$ $username$ $message$

    note: installable as Nagios plugin

### One-line Nagios Plugin Install
builds in /tmp and copies bin to /usr/local/nagios/libexec/
```agsl
curl -H 'Cache-Control: no-cache, no-store' https://raw.githubusercontent.com/helloimalemur/notify_via_discord/main/src/install_as_nagios_plugin.sh | sh
```

### Normal build
    git clone https://github.com/helloimalemur/notify_via_discord.git
    cd notify_via_discord/
    cargo build
    ./target/debug/notify_via_discord $api_url$ $username$ $message$
