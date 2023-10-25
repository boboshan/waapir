#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use log::info;
use serde_json::json;
use waapir::waapi::ak;
use wampire::{
    client::{Client, Connection},
    ArgList, Value, URI, List
};

#[tokio::main]
async fn main() {
    let connection = Connection::new("ws://127.0.0.1:8080/waapi", "realm1");

    println!("Connecting");
    let mut client = connection.connect().unwrap();

    println!("Connected");

    let options = json!({
        "return":["id"]
    });

    match client
        .call(
            URI::new(ak::wwise::ui::GET_SELECTED_OBJECTS),
            None,
            None,
            serde_json::from_value(options).unwrap()
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
