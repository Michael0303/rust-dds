use zenoh::*;
use std::convert::TryInto;

#[async_std::main]
async fn main() {
    let zenoh = Zenoh::new(net::config::default()).await.unwrap();
    let workspace = zenoh.workspace(None).await.unwrap();
    workspace.put(
        &"/demo/example/hello".try_into().unwrap(),
        "Hello World!".into()
    ).await.unwrap();
    zenoh.close().await.unwrap();
}