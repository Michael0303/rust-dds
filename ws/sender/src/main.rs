use async_std::os::unix::net::UnixStream;
use async_std::task::sleep;
use library::messages::Msg;
use library::net_protocol::Connection;
use std::io;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let futures = vec![
        sending_task(1),
        // sending_task(2),
    ];

    futures::future::join_all(futures).await;
}

async fn sending_task(index: i32) {
    println!("please enter remote IpAddr:");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read addr error!");
    let addr = String::from(line.trim());
    println!("{}", addr);
    let mut conn = Connection::new(addr, false).await;

    // println!("my socket port is {}", conn.socket.local_addr().unwrap());
    // println!("start sending");

    loop {
        let mut stream = UnixStream::connect("/tmp/robocup").await?;
        stream.write_all(b"hello world")?;

        let mut line = String::new();
        println!("please enter sending message.");
        io::stdin().read_line(&mut line).expect("read line error!");
        let line = String::from(line.trim());
        let socket_addr = conn.socket.local_addr().unwrap().to_string();
        let msg = Msg {
            id: index,
            name: line,
            testdata: 1.7414,
            from: String::from(socket_addr),
        };
        conn.send(&msg).await;
        sleep(Duration::from_secs(5)).await;
    }
}
