use library::net_protocol::{Connection, encode, decode};
use zenoh::{prelude::*, Session};

#[async_std::main]
async fn main() {
    let session = zenoh::open(config::default()).await.unwrap();  
    let futures = vec![
        receiving_task(&session),
    ];
    futures::future::join_all(futures).await;
    session.close().await.unwrap();
}

async fn receiving_task(session: &Session) {
    let mut conn = Connection::new("127.0.0.1:3333", true).await;
    println!("Listening on {}", conn.socket.local_addr().unwrap());
    loop {
        let msg = conn.recv_from().await;
        println!("recv message below:");
        println!("{:#?}", msg);
        let msg = encode(msg);
        // println!("{:#?}", msg);
        session.put("/key/robot_msg", msg).await.unwrap();
    }
}