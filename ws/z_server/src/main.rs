use futures::prelude::*;
use zenoh::prelude::*;
use library::net_protocol::{Connection, encode, decode};

#[async_std::main]
async fn main() {
    let session = zenoh::open(config::default()).await.unwrap();
    let mut subscriber = session.subscribe("/key/robot_msg").await.unwrap();
    while let Some(msg) = subscriber.next().await {
        let msg = decode(msg.value.payload.contiguous().as_ref().to_owned());
        println!("Received : {:#?}", msg);
    };
}