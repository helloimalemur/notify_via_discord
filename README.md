# Notify_Via_Discord
    Rust micro-app used to send messages to Discord web-hook ~(Nagios compatible)~

    sends POST request to specified Discord webhook url after building and appending json with specified username and message 

    Format: ./notify_discord $api_url$ $username$ $message$

    note: installable as Nagios plugin

### One-line Nagios Plugin Install
builds in /tmp and copies bin to /usr/local/nagios/libexec/
```agsl
curl https://raw.githubusercontent.com/helloimalemur/notify_via_discord/main/src/install_as_nagios_plugin.sh | sh
```
