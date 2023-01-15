// https://docs.rs/serde_json/latest/serde_json/
use serde_json::{json, Result, Value};



pub fn jsonify(username: &str, message: &str) -> Result<Value> {
    let data = json!({
    "username": username,
    "content": message,
    });


    println!("{}", data.to_string());

    Ok(data)
}
