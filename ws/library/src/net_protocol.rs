use async_std::net::{UdpSocket};
use async_std::net::ToSocketAddrs;
use prost::Message;

use crate::messages::Msg;

pub struct Connection {
    pub socket: UdpSocket,
}


impl Connection {
    pub async fn new<A>(addr: A, server: bool) -> Self
    where
        A: ToSocketAddrs
    {
        let mut addr_iter = addr.to_socket_addrs().await.expect("err here!");
        let addr = addr_iter.next().unwrap();
        assert!(addr_iter.next().is_none(), "only one address is allowed");
        let mut bind_addr_iter = "127.0.0.1:0".to_socket_addrs().await.unwrap();
        let mut bind_addr = bind_addr_iter.next().unwrap();
        if server {
            bind_addr = addr;
        }
        let socket = UdpSocket::bind(bind_addr).await.unwrap();
        if !server {
            socket.connect(addr).await.unwrap();
        }
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

    pub async fn send_to<A>(&mut self, msg: &Msg, addr: &A) 
    where
        A: ToSocketAddrs
    {
        let msg_bytes: Vec<u8> = {
            let mut buf = Vec::with_capacity(msg.encoded_len());
            msg.encode(&mut buf).unwrap();
            buf
        };
        let len: u64 = msg_bytes.len() as u64;
        let len_bytes = len.to_le_bytes();


        let mut len_bytes = len_bytes.as_slice();
        while !len_bytes.is_empty() {
            let sent_len = self.socket.send_to(&len_bytes, addr).await.expect("sending length error");
            len_bytes = &len_bytes[sent_len..];
        }

        let mut msg_bytes = msg_bytes.as_slice();
        while !msg_bytes.is_empty() {
            let sent_len = self.socket.send_to(&msg_bytes, addr).await.expect("sending length error");
            msg_bytes = &msg_bytes[sent_len..];
        }
        
    }

    pub async fn recv_from(&mut self) -> Msg {
        
        let msg_len = {
            let mut len_bytes = [0u8; 8];
            let mut slice = len_bytes.as_mut_slice();
            while !slice.is_empty() {
                let (len, _) = self.socket.recv_from(slice).await.unwrap();
                slice = &mut slice[len..];
            }
    
    
            u64::from_le_bytes(len_bytes)
        };
    
        assert!(msg_len <= 0x1000, "message is too large");
    
        let msg_bytes: Vec<u8> = {
            let mut msg_bytes = vec![0u8; msg_len as usize];
            
            let mut slice = msg_bytes.as_mut_slice();
            
            while !slice.is_empty() {
                let (len, _) = self.socket.recv_from(slice).await.unwrap();
                slice = &mut slice[len..];
            }
            
            msg_bytes
        };
    
        let msg = Msg::decode(msg_bytes.as_slice()).unwrap();
        msg
    }

}

pub fn encode(msg: Msg) -> Vec<u8> {
    let mut buf = Vec::with_capacity(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    buf
}

pub fn decode(msg_bytes: Vec<u8>) -> Msg {
    let msg = Msg::decode(msg_bytes.as_slice()).unwrap();
    msg
}