use async_std::os::unix::net::{UnixListener, UnixStream};
use async_std::prelude::*;
use futures::TryFutureExt;
// use library::net_protocol::{decode, Connection};
// use rmps::{Deserializer, Serializer};
// use serde::{Deserialize, Serialize};
use std::fs;
use std::str::FromStr;
use zenoh::Session;

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
    let futures = vec![receiving_task(&session)];
    futures::future::join_all(futures).await;
    session.close().await.unwrap();
}

async fn receiving_task(session: &Session) -> async_std::io::Result<()> {
    let listener = UnixListener::bind("/tmp/robocup")
        .await
        .expect("unix socket connect error!");
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        loop {
            let mut msg_stream = vec![0; 1024];
            let length = stream.read(&mut msg_stream).await?;
            if length == 0 {
                break;
            };
            println!("recv message ");
            // session.put("/key/robot_msg", msg_stream).await.unwrap();
        }
    }
    Ok(())
}
