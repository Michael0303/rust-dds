use library::net_protocol::Connection;
use async_std::task::sleep;
use std::time::Duration;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
// use zenoh::*;
// use std::convert::TryInto;

#[async_std::main]
async fn main() {
    
    let futures = vec![
        receiving_task(0),
        receiving_task(1),
    ];

    futures::future::join_all(futures).await;
}

async fn receiving_task(index: i32) {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), (3333 + index) as u16);
    let mut conn = Connection::new(addr, true).await;
    println!("Listening on {}", conn.socket.local_addr().unwrap());
    loop {
        let msg = conn.recv_from().await;
        println!("recv message below:");
        println!("{:#?}", msg);
        sleep(Duration::from_secs(5)).await;
    }
}