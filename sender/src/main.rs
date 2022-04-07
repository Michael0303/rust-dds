use std::net::{UdpSocket};
use std::env;
use std::io;

struct Message {
    id: u32,
    name: String,
    sample1: f64,
}


fn listen(socket: &UdpSocket, mut buf: &mut [u8]) -> usize {
    let mut bytes: [u8; 8] = [0; 8];
    let (num_of_bytes, src_ip) = socket.recv_from(&mut bytes).expect("not receiving data");
    let len = usize::from_ne_bytes(bytes);
    let (num_of_bytes, src_ip) = socket.recv_from(&mut buf).expect("not receiving data");
    println!("recv {} bytes from {}", num_of_bytes, src_ip);
    len
}

fn send(socket: &UdpSocket, msg: &[u8], receiver: &str) -> usize {
    let len: usize = msg.len();
    let bytes = len.to_ne_bytes();
    let result = socket.send_to(&bytes, receiver).expect("sending length error");

    let result = socket.send_to(&msg, receiver).expect("sending data error");
    println!("sending {} bytes to {}", result, receiver);
    result
}

fn init(sender_port: &str, receiver_port: &str) -> (String, String, UdpSocket) {
    let mut sender_ipaddr = String::from("127.0.0.1:");
    let mut receiver_ipaddr = String::from("127.0.0.1:");
    sender_ipaddr.push_str(sender_port);
    receiver_ipaddr.push_str(receiver_port);
    println!("sender: {}, receiver: {}", sender_ipaddr, receiver_ipaddr);

    let socket = UdpSocket::bind(&sender_ipaddr).expect("couldn't bind to address");

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
        let mut msg = String::new();
        println!("please enter sending message.");
        io::stdin().read_line(&mut msg).expect("read line error!");
        let msg = msg.trim().as_bytes();
        send(&socket, &msg, &receiver);

        let len = listen(&socket, &mut buf);
        let buf = &mut buf[..len];
        println!("recv feedback OK.");
    }
    
}