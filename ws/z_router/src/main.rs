use futures::prelude::*;
use zenoh::prelude::*;
use zenoh::scouting::WhatAmI;

let mut receiver = zenoh::scout(WhatAmI::Peer | WhatAmI::Router, config::default()).await.unwrap();
while let Some(hello) = receiver.next().await {
    println!("{}", hello);
}
