use std::str::FromStr;
use std::fs;
use futures::prelude::*;
use zenoh::prelude::*;
use library::net_protocol::decode;

#[async_std::main]
async fn main() {
    use zenoh::config::{ConnectConfig, EndPoint};
    let filename = "router_config";
    let routerip = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("zenoh_router IpAddr is {}", routerip);
    let mut config = zenoh::config::default();
    config
        .set_connect(ConnectConfig {
            endpoints: vec![EndPoint::from_str(&routerip).expect("router IpAddr error!")],
        });

    let session = zenoh::open(config).await.unwrap();
    let mut subscriber = session.subscribe("/key/robot_msg").await.unwrap();
    while let Some(msg) = subscriber.next().await {
        let msg = decode(msg.value.payload.contiguous().as_ref().to_owned());
        println!("Received : {:#?}", msg);
    };
}