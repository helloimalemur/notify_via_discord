# notify_via_discord
    Rust micro-app used to send messages to Discord web-hook (Nagios compatible)

    sends POST request to specified Discord webhook url after building and appending json with specified username and message 

    Format: ./notify_discord $api_url$ $username$ $message$

    note: installable as Nagios plugin
