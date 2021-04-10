// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

//! The entry point to using the networking interface as an out-of-process service.

use std::time::Duration;

use ethereum::Transaction;
use networking::{Config, NetworkInterface};
use runtime::{expose, ingest, receive};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create an instance of the networking interface
  let network = NetworkInterface::new(Config::default()).await?;

  // forward all produced messages of type Transaction to a  unix socket
  let _tx_publisher = expose::<Transaction>(&network, "/var/run/oe/net.ipc");
  let _tx_consumer = ingest::<Transaction>(&network, "/var/run/oe/net.ipc");

  let t1 = async {
    let net = network.clone();
    loop {
      println!("devp2p peers: {}", net.num_peers());
      tokio::time::sleep(Duration::from_secs(3)).await;
    }
  };

  let t2 = async {
    let net = network.clone();
    while let Ok(tx) = receive::<Transaction>(&net).await {
      println!("received transaction from devp2p: {:?}", tx);
    }
  };

  futures::future::join(t1, t2).await;
  network.shutdown().await?;
  Ok(())
}
