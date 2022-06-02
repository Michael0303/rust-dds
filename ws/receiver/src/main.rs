use library::net_protocol::{decode, encode, Connection};
use std::fs;
use async_std::os::unix::net::UnixStream;;
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

async fn receiving_task(session: &Session) {
    let mut conn = Connection::new("127.0.0.1:3333", true).await;
    println!("Listening on {}", conn.socket.local_addr().unwrap());
    loop {
        let mut stream = UnixStream::connect("/tmp/robocup").await?;
        // let msg = conn.recv_from().await;
        let mut msg_stream = String::new();
        stream.read_to_string(&mut msg_stream)?;
        // println!("recv message below:");
        // println!("{:#?}", msg_stream);
        let msg = encode(msg);
        // println!("{:#?}", msg);
        session.put("/key/robot_msg", msg).await.unwrap();
    }
}
