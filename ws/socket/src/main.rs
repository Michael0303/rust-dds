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
    joint: joint,
    touch: touch,
    teset: teset,
}
#[derive(Debug, PartialEq, Deserialize)]
struct joint {
    a: f32,
    b: f32,
    c: f32,
}
#[derive(Debug, PartialEq, Deserialize)]
struct touch {
    a: f32,
    b: f32,
    c: f32,
}
#[derive(Debug, PartialEq, Deserialize)]
struct teset {
    a: f32,
    b: String,
    c: f32,
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
        let mut msg_stream = vec![0; 256];
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
