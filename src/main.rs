#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use log::info;
use serde_json::json;
use waapir::waapi::ak;
use wampire::{
    client::{Client, Connection},
    ArgList, Value, URI,
};

#[tokio::main]
async fn main() {
    let connection = Connection::new("ws://127.0.0.1:8080/waapi", "realm1");

    println!("Connecting");
    let mut client = connection.connect().unwrap();

    println!("Connected");

    let kwargs = json!({
        "from": {
            "id" : ["{6EE88ECF-9723-4DA1-82B4-E3B00FD21314}"]
        }
    });

    let options = json!({
        "return" : ["type", "id"]
    });

    match client
        .call(
            URI::new(ak::wwise::core::object::GET),
            None,
            serde_json::from_value(kwargs).ok(),
            serde_json::from_value(options).ok(),
        )
        .await
    {
        Ok(result) => {
            println!("Got result: {:#?}", result.1);
        }
        Err(err) => {
            println!("Error making the call: {:?}", err);
        }
    };
    client.shutdown();
}
