use std::io;
use library::net_protocol::Connection;
use library::messages::Msg;
use async_std::task::sleep;
use std::time::Duration;


#[async_std::main]
async fn main() {
    let futures = vec![
        sending_task(1),
        sending_task(2),
    ];

    futures::future::join_all(futures).await;
}

async fn sending_task(index: i32) {
    let mut conn = Connection::new("127.0.0.1:3333", false).await;
    println!("my socket port is {}", conn.socket.local_addr().unwrap());
    println!("start receiving");
    
    loop {
        let mut line = String::new();
        println!("please enter sending message.");
        io::stdin().read_line(&mut line).expect("read line error!");
        let line = String::from(line.trim());
        socket_addr = conn.socket.local_addr().unwrap().parse();
        let msg = Msg {
            id: index,
            name: line,
            testdata: 1.7414,
            from: String::from(socket_addr),
        };
        conn.send(&msg).await;
        sleep(Duration::from_secs(10)).await;
    }
    
}