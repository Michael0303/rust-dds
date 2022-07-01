use library::net_protocol::*;

use std::fs;
use std::path::Path;

#[async_std::main]
async fn main() {
    let futures = vec![sending_task()];
    futures::future::join_all(futures).await;
}
async fn sending_task() -> async_std::io::Result<()> {
    let path: &Path = Path::new("/tmp/zenohsocket");
    match Path::exists(path) {
        true => {
            println!("remove /tmp/zenohsocket");
            fs::remove_file("a.txt")?;
        }
        false => (),
    }

    let mut conn = Connection::new(path).await;
    loop {
        let msg_stream = conn.recv().await;
        if msg_stream.length == 0 {
            break;
        };
        let msg = decode(msg_stream);
        println!("{:#?}", msg);
    }
    conn.shutdown().await;
    return Ok(());
}
