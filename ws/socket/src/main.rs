use library::net_protocol::*;
use std::fs;
use std::path::Path;
use zenoh::Session;

#[async_std::main]
async fn main() {
    let session = zenoh_conn().await;
    let futures = vec![sending_task(&session)];
    futures::future::join_all(futures).await;
    session.close().await.unwrap();
}
async fn sending_task(session: &Session) -> async_std::io::Result<()> {
    let path: &Path = Path::new("/tmp/zenohsocket");
    match Path::exists(path) {
        true => {
            println!("remove /tmp/zenohsocket");
            fs::remove_file("/tmp/zenohsocket")?;
        }
        false => (),
    }

    let mut conn = Connection::new(path).await;
    loop {
        let msg_stream = conn.recv().await;
        if msg_stream.length == 0 {
            break;
        };
        let msg_bytes: &[u8] = &(msg_stream.bytes);
        session.put("/key/robot_msg", msg_bytes).await.unwrap();
        // let msg = decode(msg_stream);
        // println!("{:#?}", msg);
    }
    conn.shutdown().await;
    return Ok(());
}
