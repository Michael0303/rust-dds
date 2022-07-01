use async_std::os::unix::net::{UnixListener, UnixStream};
use async_std::prelude::*;
use std::net::Shutdown;
use std::path::Path;

extern crate rmp_serde as rmps;
extern crate serde;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

pub struct Connection {
    pub stream: UnixStream,
}

pub struct Msg_stream {
    pub length: usize,
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Msg {
    pub Status: [i32; 25],
    pub Stiffness: [f32; 25],
    pub Accelerometer: [f32; 3],
    pub Battery: [f32; 4],
    pub Current: [f32; 25],
    pub Touch: [f32; 14],
    pub FSR: [f32; 8],
    pub Angles: [f32; 2],
    pub Position: [f32; 25],
    pub Sonar: [f32; 2],
    pub RobotConfig: [String; 4],
    pub Gyroscope: [f32; 3],
    pub Temperature: [f32; 25],
}

impl Connection {
    // pub async fn new<A>(addr: A, server: bool) -> Self
    // where
    //     A: ToSocketAddrs,
    // {
    //     let mut addr_iter = addr.to_socket_addrs().await.expect("err here!");
    //     let addr = addr_iter.next().unwrap();
    //     assert!(addr_iter.next().is_none(), "only one address is allowed");
    //     let mut bind_addr_iter = "127.0.0.1:0".to_socket_addrs().await.unwrap();
    //     let mut bind_addr = bind_addr_iter.next().unwrap();
    //     if server {
    //         bind_addr = addr;
    //     }
    //     let socket = UdpSocket::bind(bind_addr).await.unwrap();
    //     if !server {
    //         socket.connect(addr).await.unwrap();
    //     }
    //     Self { socket }
    // }
    // pub async fn new() -> Self {
    //     let mut stream = UnixStream::connect("/tmp/robocup")
    //         .await
    //         .expect("unix socket connect error!");
    //     Self { stream }
    // }
    pub async fn new(path: &Path) -> Self {
        let listener = UnixListener::bind(path).await.expect("binding error");
        let (mut stream, addr) = listener.accept().await.expect("listen error");
        println!("Got a client: {:?} - {:?}", stream, addr);
        Self { stream }
    }
    pub async fn recv(&mut self) -> Msg_stream {
        let mut bytes = vec![0; 896];
        let length = self.stream.read(&mut bytes).await.unwrap();
        println!("recv message length = {}", length);
        Msg_stream { length, bytes }
    }

    pub async fn shutdown(&mut self) {
        self.stream
            .shutdown(Shutdown::Both)
            .expect("shut down error!");
    }
}

pub fn decode(msg_stream: Msg_stream) -> Msg {
    let buf = &msg_stream.bytes[0..msg_stream.length];
    let msg: Msg = rmps::from_slice(buf).unwrap();
    msg
}
