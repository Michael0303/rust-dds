pub mod net_protocol {
    use std::net::{UdpSocket};

    pub enum Who {
        Sender,
        Receiver,
    }
    
    pub fn listen(socket: &UdpSocket, mut buf: &mut [u8]) -> usize {
        let mut bytes: [u8; 8] = [0; 8];
        socket.recv_from(&mut bytes).expect("not receiving length");
        let len = usize::from_ne_bytes(bytes);
        let (num_of_bytes, src_ip) = socket.recv_from(&mut buf).expect("not receiving data");
        println!("recv {} bytes from {}", num_of_bytes, src_ip);
        len
    }

    pub fn send(socket: &UdpSocket, msg: &[u8], sentto: &str) -> usize {
        let len: usize = msg.len();
        let bytes = len.to_ne_bytes();
        socket.send_to(&bytes, sentto).expect("sending length error");

        let result = socket.send_to(&msg, sentto).expect("sending data error");
        println!("sending {} bytes to {}", result, sentto);
        result
    }

    pub fn init(sender_port: &str, receiver_port: &str, who: Who) -> (String, String, UdpSocket) {
        let mut sender_ipaddr = String::from("127.0.0.1:");
        let mut receiver_ipaddr = String::from("127.0.0.1:");
        sender_ipaddr.push_str(sender_port);
        receiver_ipaddr.push_str(receiver_port);
        println!("sender: {}, receiver: {}", sender_ipaddr, receiver_ipaddr);

        let socket = match who {
            Who::Sender => UdpSocket::bind(&sender_ipaddr).expect("couldn't bind to address"),
            Who::Receiver => UdpSocket::bind(&receiver_ipaddr).expect("couldn't bind to address"),
        };

        let result = (sender_ipaddr, receiver_ipaddr, socket);
        result
    }
}

use prost::Message;
use std::io::Cursor;

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/library.message.rs"));
}

pub fn serialize(msg: &messages::Msg) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    msg.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize(buf: &[u8]) -> Result<messages::Msg, prost::DecodeError> {
    messages::Msg::decode(&mut Cursor::new(buf))
}
