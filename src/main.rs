use serde::{Deserialize, Serialize};
use std::error::Error;
use wamp_async::{
    try_into_any_value, Client, ClientConfig, ClientRole, SerializerType, WampKwArgs,
};
mod waapi;
use waapi::Ak;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (mut client, (evt_loop, _rpc_evt_queue)) = Client::connect(
        "ws://localhost:8080/waapi",
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

    tokio::spawn(evt_loop);

    println!("Joining realm");
    client.join_realm("realm1").await?;

    match client
        .call(Ak::WwiseCoreGetInfo.to_string(), None, None)
        .await
    {
        Ok((res_args, res_kwargs)) => {
            println!("Got result: {:#?}", res_kwargs);
        }
        Err(e) => println!("Error calling procedure: {:?}", e),
    };
    Ok(())
}
