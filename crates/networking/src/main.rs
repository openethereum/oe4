// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

//! The entry point to using the networking interface as an out-of-process service.

use std::time::Duration;

use ethereum::Transaction;
use networking::{Config, NetworkInterface};
use runtime::{expose, ingest, receive, send};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create an instance of the networking interface
  let network = NetworkInterface::new(Config::default()).await?;

  // forward all transactions of type Transaction to a unix socket that
  // were produced by local client components to the p2p network
  let _tx_from_client_to_world = ingest::<Transaction>(&network, "/var/run/oe4/net.ipc");

  // forward all produced messages of type Transaction to a unix socket that
  // arrived from the p2p network to other client components
  let _tx_from_world_to_client = expose::<Transaction>(&network, "/var/run/oe4/net.ipc");

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
      //send(&tx_from_world_to_client, tx).await; // continue here tomorrow
    }
  };

  futures::future::join(t1, t2).await;
  network.shutdown().await?;
  Ok(())
}
