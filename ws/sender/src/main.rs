use async_std::os::unix::net::UnixStream;
use async_std::prelude::*;
use async_std::task::sleep;
use library::messages::{Joint, Robot, Sonar, Touch};
use library::net_protocol::Connection;
// use rmps::{Deserializer, Serializer};
// use serde::{Deserialize, Serialize};
use std::io;
use std::net::Shutdown;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let futures = vec![
        sending_task(1),
        // sending_task(2),
    ];

    futures::future::join_all(futures).await;
}

async fn sending_task(index: i32) {
    // println!("please enter remote IpAddr:");
    // let mut line = String::new();
    // io::stdin().read_line(&mut line).expect("read addr error!");
    // let addr = String::from(line.trim());
    // println!("{}", addr);
    // let mut conn = Connection::new(addr, false).await;

    // println!("my socket port is {}", conn.socket.local_addr().unwrap());
    // println!("start sending");
    let mut stream = UnixStream::connect("/tmp/robocup")
        .await
        .expect("unix socket connect error!");
    loop {
        let mut line = String::new();
        println!("please enter sending message.");
        io::stdin().read_line(&mut line).expect("read line error!");
        let line = String::from(line.trim());
        let slice = &line[0..4];
        if slice == "exit" {
            break;
        }
        let line = line.as_bytes();

        stream.write_all(&line).await.expect("read line error!");
        // sleep(Duration::from_secs(5)).await;
    }

    stream.shutdown(Shutdown::Both).expect("shut down error!");
}
