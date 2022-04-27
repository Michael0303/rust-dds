use library::net_protocol::Connection;

#[async_std::main]
async fn main() {
    let futures = vec![
        receiving_task(),
    ];

    futures::future::join_all(futures).await;
}

async fn receiving_task() {
    let mut conn = Connection::new("127.0.0.1:3333", true).await;
    println!("Listening on {}", conn.socket.local_addr().unwrap());
    loop {
        let msg = conn.recv_from().await;
        println!("recv message below:");
        println!("{:#?}", msg);
    }
}