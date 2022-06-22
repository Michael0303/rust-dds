use async_std::os::unix::net::UnixStream;
use async_std::prelude::*;
use std::net::Shutdown;

extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize)]
struct Test {
    Status: [i32; 25],
    Stiffness: [f32; 25],
    Accelerometer: [f32; 3],
    Battery: [f32; 4],
    Current: [f32; 25],
    Touch: [f32; 14],
    FSR: [f32; 8],
    Angles: [f32; 2],
    Position: [f32; 25],
    Sonar: [f32; 2],
    RobotConfig: [String; 4],
    Gyroscope: [f32; 3],
    Temperature: [f32; 25],
}

#[async_std::main]
async fn main() {
    let futures = vec![sending_task()];
    futures::future::join_all(futures).await;
}

async fn sending_task() -> async_std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/robocup")
        .await
        .expect("unix socket connect error!");
    loop {
        let mut msg_stream = vec![0; 896];
        let length = stream.read(&mut msg_stream).await?;
        if length == 0 {
            break;
        };
        println!("recv message length = {}", length);
        let buf = &msg_stream[0..length];
        let test: Test = rmps::from_slice(buf).unwrap();
        println!("{:#?}", test);
    }

    stream.shutdown(Shutdown::Both).expect("shut down error!");
    return Ok(());
}
