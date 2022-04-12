use std::env;
use std::io;
use library::net_protocol::{init, listen, send, Who};
use library::{serialize, deserialize, messages};

fn main() {
    let args: Vec<String> = env::args().collect();
    let sender_port = &args[1];
    let receiver_port = &args[2]; 
    let (sender, receiver, socket) = init(sender_port, receiver_port, Who::Sender);

    let mut buf = [0; 100];

    println!("start receiving");
   
    loop {
        let mut msg = String::new();
        println!("please enter sending message.");
        io::stdin().read_line(&mut msg).expect("read line error!");
        // let msg = msg.trim().as_bytes();
        let msg = messages::Msg{
            id: 7414,
            name: String::from("naov6"),
            testdata: 1.7414,
        };
        let msg = serialize(&msg);
        send(&socket, &msg, &receiver);

        let len = listen(&socket, &mut buf);
        let buf = &mut buf[..len];
        println!("recv feedback OK.");
    }
    
}