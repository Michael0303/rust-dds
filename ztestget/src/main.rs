use zenoh::*;
use futures::prelude::*;
use std::convert::TryInto;

#[async_std::main]
async fn main() {
    let zenoh = Zenoh::new(net::config::default()).await.unwrap();
    let workspace = zenoh.workspace(None).await.unwrap();
    let mut change_stream =
        workspace.subscribe(&"/demo/example/**".try_into().unwrap()).await.unwrap();
    while let Some(change) = change_stream.next().await {
        println!(">> {:?} for {} : {:?} at {}",
            change.kind, change.path, change.value, change.timestamp
        )
    }
    change_stream.close().await.unwrap();
    zenoh.close().await.unwrap();
}