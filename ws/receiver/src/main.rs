use std::env;
use library::net_protocol::{init, listen, send, Who};
use library::{serialize, deserialize, messages};

fn main() {
    let args: Vec<String> = env::args().collect();
    let sender_port = &args[1];
    let receiver_port = &args[2];
    let (sender, receiver, socket) = init(sender_port, receiver_port, Who::Receiver);
    
    let mut buf = [0; 100];
    println!("start receiving");
    loop {
        let len = listen(&socket, &mut buf);
        let buf = &mut buf[..len];
        println!("recv message below:");
        let msg = deserialize(buf).unwrap();
        println!("{:#?}", msg);

        let msg = String::from("Ok");
        let msg = msg.as_bytes();
        send(&socket, &msg, &sender);
    }
}