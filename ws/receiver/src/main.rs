use library::net_protocol::{Connection, encode, decode};
use zenoh::Session;
use std::fs;
use std::str::FromStr;

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
    let futures = vec![
        receiving_task(&session),
    ];
    futures::future::join_all(futures).await;
    session.close().await.unwrap();
}

async fn receiving_task(session: &Session) {
    let mut conn = Connection::new("127.0.0.1:3333", true).await;
    println!("Listening on {}", conn.socket.local_addr().unwrap());
    loop {
        let msg = conn.recv_from().await;
        println!("recv message below:");
        println!("{:#?}", msg);
        let msg = encode(msg);
        // println!("{:#?}", msg);
        session.put("/key/robot_msg", msg).await.unwrap();
    }
}