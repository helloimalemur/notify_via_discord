//https://github.com/serde-rs/json
// James Koonts
extern crate core;
use std::{env, process};
use tokio;
use reqwest::ClientBuilder;
use reqwest::header::{CONTENT_TYPE};
use serde_json::Value;

mod to_json;


#[tokio::main]
async fn main() {
    // get arguments
    let args: Vec<String> = env::args().collect();
    // get first argument
    let api_url = match args.get(1) {
        Some(key) => key,
        None => ""
    };
    // get second argument
    let username = match args.get(2) {
        Some(k) => k,
        None => ""
    };
    // get third argument
    let message = match args.get(3) {
        Some(k) => k,
        None => ""
    };
    //check for no arguments
    if api_url == "" || message == "" || username == "" {
        // exit code 3 if no args
        no_arg_error()
    } else {
        // continue normally if args present
        let json_message = match to_json::jsonify(username, message) {
            Ok(j) => j,
            Err(_e) => process::exit(3)
        };
        push_message(api_url, json_message).await;
    }

}
pub async fn push_message(api_url: &str, json_message: Value) {
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .no_gzip()
        .build();

    let response = match client {
        Ok(r) => r
            .post(api_url)
            .header(CONTENT_TYPE, "application/json")
            .json(&json_message)
            .send().await, //
        Err(_e) => process::exit(3)
    };

    let result_text = match response {
        Ok(r) => r.text().await,
        Err(_e) => process::exit(3)
    };

    match result_text.as_ref() {
     Ok(t) => match t.contains("") {
         true => success(t),
         false => failure() },
     Err(_e) => process::exit(3)
    };
}

fn success(text: &str) {
    println!("{} {} {} {}", "message sent successfully", "|", "OK", text);
    process::exit(0)
}

fn failure() {
    println!("{} {} {}", "message send failed", "|", "CRITICAL");
    process::exit(2)
}


fn no_arg_error() {
    println!("{}", "No Arguments");
    println!("{}", "Format: ./notify_discord $api_url$ $username$ $message$");
    process::exit(3);
}
