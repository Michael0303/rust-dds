use std::net::{UdpSocket};
use std::env;
use protocol;
struct Message {
    id: u32,
    name: String,
    sample1: f64,
}


fn listen(socket: &UdpSocket, mut buf: &mut [u8]) -> usize {
    let mut bytes: [u8; 8] = [0; 8];
    socket.recv_from(&mut bytes).expect("not receiving data");
    let len = usize::from_ne_bytes(bytes);
    let mut byte_count = 0;
    while byte_count < len {
        let (num_of_bytes, src_ip) = socket.recv_from(&mut buf).expect("not receiving data");
        println!("recv {} bytes from {}", num_of_bytes, src_ip);
        byte_count += num_of_bytes;
    }
    len
}

fn send(socket: &UdpSocket, msg: &[u8], sender: &str) -> usize {
    let len: usize = msg.len();
    let bytes = len.to_ne_bytes();
    socket.send_to(&bytes, sender).expect("sending length error");
    let mut byte_count = 0;
    while byte_count < len {
        let num_of_bytes = socket.send_to(&msg, sender).expect("sending data error");
        println!("sending {} bytes to {}", num_of_bytes, sender);
        byte_count += num_of_bytes;
    }
    len
}

fn init(sender_port: &str, receiver_port: &str) -> (String, String, UdpSocket) {
    let mut sender_ipaddr = String::from("127.0.0.1:");
    let mut receiver_ipaddr = String::from("127.0.0.1:");
    sender_ipaddr.push_str(sender_port);
    receiver_ipaddr.push_str(receiver_port);
    println!("sender: {}, receiver: {}", sender_ipaddr, receiver_ipaddr);

    let socket = UdpSocket::bind(&receiver_ipaddr).expect("couldn't bind to address");

    let result = (sender_ipaddr, receiver_ipaddr, socket);
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let sender_port = &args[1];
    let receiver_port = &args[2]; 
    let (sender, receiver, socket) = init(sender_port, receiver_port);
    
    let mut buf = [0; 100];

    println!("start receiving");
    loop {
        let len = listen(&socket, &mut buf);
        let buf = &mut buf[..len];
        println!("recv message below:");
        let msg = std::str::from_utf8(&buf).unwrap();
        println!("{}", msg);

        let msg = String::from("Ok");
        let msg = msg.as_bytes();
        send(&socket, &msg, &sender);
    }
    protocol::somefn();
}