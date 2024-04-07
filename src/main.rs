#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::error::Error;
use waapir::waapi::ak::{self, wwise};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use wamp_async::{
    try_into_any_value, Client, ClientConfig, ClientRole, SerializerType, WampKwArgs,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct MyStruct {
    field1: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct WwiseInfo {
    api_version: u32,
    branch: String,
    configuration: String,
    copyright: String,
    directories: Directories,
    display_name: String,
    is_command_line: bool,
    platform: String,
    process_id: u32,
    process_path: String,
    session_id: String,
    version: Version,
}

#[derive(Debug, Deserialize, Serialize)]
struct Directories {
    authoring: String,
    bin: String,
    help: String,
    install: String,
    user: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Version {
    build: u32,
    display_name: String,
    major: u32,
    minor: u32,
    nickname: String,
    schema: u32,
    year: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the server
    let (mut client, (evt_loop, _rpc_evt_queue)) = Client::connect(
        "ws://127.0.0.1:8080/waapi",
        Some(
            ClientConfig::default()
                .set_ssl_verify(false)
                // Restrict our roles
                .set_roles(vec![ClientRole::Caller])
                // Only use Json serialization
                .set_serializers(vec![SerializerType::Json]),
        ),
    )
    .await?;
    println!("Connected !!");

    // Spawn the event loop
    tokio::spawn(evt_loop);

    println!("Joining realm");
    client.join_realm("realm1").await?;

    println!("Calling");
    match client.call(wwise::core::GET_INFO, None, None).await {
        Ok((res_args, res_kwargs)) => {
            if let Some(kwargs) = res_kwargs {
                if let Ok(value) = try_into_any_value(kwargs) {
                    // println!("Received JSON: {}", serde_json::to_string_pretty(&value).unwrap());
                    // let wwise_info: WwiseInfo = serde_json::from_value(value).unwrap();
                    if let Ok(wwise_info) = serde_json::from_value::<WwiseInfo>(value) {
                        println!("Version display name: {:?}", wwise_info.version.display_name);
                    } else {
                        println!("Error deserializing WwiseInfo");
                    }
                }
            }
        }
        Err(e) => {
            println!("Error calling ({:?})", e);
        }
    }

    println!();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    println!("Leaving realm");
    client.leave_realm().await?;

    client.disconnect().await;
    Ok(())
}
