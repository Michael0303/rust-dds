use async_std::net::{UdpSocket};
use async_std::net::ToSocketAddrs;
use prost::Message;

use crate::messages::Msg;

pub struct Connection {
    socket: UdpSocket,
}

impl Connection {
    pub async fn new<A>(addr: A) -> Self
    where
        A: ToSocketAddrs
    {
        let mut addr_iter = addr.to_socket_addrs().await.unwrap();
        let addr = addr_iter.next().unwrap();
        assert!(addr_iter.next().is_none(), "only one address is allowed");

        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        socket.connect(addr).await.unwrap();

        Self {
            socket,
        }
    }
    

    pub async fn send(&mut self, msg: &Msg) {
        let msg_bytes: Vec<u8> = {
            let mut buf = Vec::with_capacity(msg.encoded_len());
            msg.encode(&mut buf).unwrap();
            buf
        };
        let len: u64 = msg_bytes.len() as u64;
        let len_bytes = len.to_le_bytes();


        let mut len_bytes = len_bytes.as_slice();
        while !len_bytes.is_empty() {
            let sent_len = self.socket.send(&len_bytes).await.expect("sending length error");
            len_bytes = &len_bytes[sent_len..];
        }

        let mut msg_bytes = msg_bytes.as_slice();
        while !msg_bytes.is_empty() {
            let sent_len = self.socket.send(&msg_bytes).await.expect("sending length error");
            msg_bytes = &msg_bytes[sent_len..];
        }
        
    }


    pub async fn recv(&mut self) -> Msg {
        
        let msg_len = {
            let mut len_bytes = [0u8; 8];
            let mut slice = len_bytes.as_mut_slice();
            while !slice.is_empty() {
                let len = self.socket.recv(slice).await.unwrap();
                slice = &mut slice[len..];
            }


            u64::from_le_bytes(len_bytes)
        };

        assert!(msg_len <= 0x1000, "message is too large");

        let msg_bytes: Vec<u8> = {
            let mut msg_bytes = vec![0u8; msg_len as usize];
            
            let mut slice = msg_bytes.as_mut_slice();
            
            while !slice.is_empty() {
                let len = self.socket.recv(slice).await.unwrap();
                slice = &mut slice[len..];
            }
            
            msg_bytes
        };

        let msg = Msg::decode(msg_bytes.as_slice()).unwrap();
        msg
    }

}
