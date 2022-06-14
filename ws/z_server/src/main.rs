use futures::prelude::*;
use library::net_protocol::decode;
use rmp_serde::*;
use std::fs;
use std::str::FromStr;
use zenoh::prelude::*;

#[async_std::main]
async fn main() {
    use zenoh::config::{ConnectConfig, EndPoint};
    let filename = "router_config";
    let routerip = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("zenoh_router IpAddr is {}", routerip);
    let mut config = zenoh::config::default();
    config.set_connect(ConnectConfig {
        endpoints: vec![EndPoint::from_str(&routerip).expect("router IpAddr error!")],
    });

    let session = zenoh::open(config).await.unwrap();
    let mut subscriber = session.subscribe("/key/robot_msg").await.unwrap();
    while let Some(msg) = subscriber.next().await {
        let msg_bytes = msg.value.payload.contiguous().as_ref().to_owned();
        let msg = rmp_serde::from_read(&*msg_bytes).unwrap();
        println!("Received : {:#?}", msg);
    }
}
