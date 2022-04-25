use std::env;
use std::io;
use library::net_protocol::Connection;
use library::messages::Msg;


#[async_std::main]
async fn main() {
    let futures = vec![
        sending_task(),
        sending_task(),
        sending_task(),
        sending_task(),
        sending_task(),
    ];

    futures::future::join_all(futures).await;
}

async fn sending_task() {
    let args: Vec<String> = env::args().collect();
    let remote_addr = &args[1];
    let mut conn = Connection::new(remote_addr).await;

    println!("start receiving");
   
    loop {
        let mut line = String::new();
        println!("please enter sending message.");
        io::stdin().read_line(&mut line).expect("read line error!");
        let line = String::from(line.trim());

        let msg = Msg {
            id: 7414,
            name: line,
            testdata: 1.7414,
        };

        conn.send(&msg).await;
        
    }
    
}