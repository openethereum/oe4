// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

//! The entry point to using the networking interface as an out-of-process service.

use std::time::Duration;

use ethereum::Transaction;
use networking::{Config, NetworkInterface};
use runtime::{
  agent::Agent,
  nodes::ipc::{expose, ingest},
  receive,
};

async fn test() -> Result<(), Box<dyn std::error::Error>> {
  let network = Agent::new(NetworkInterface::new(Config::default()).await?).await?;

  runtime::send(&*network, Transaction::default()).await;

  network.await;

  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create an instance of the networking interface
  test().await?;
  let network = NetworkInterface::new(Config::default()).await?;

  {
    // forward all transactions of type Transactionwere produced by local client
    // components to a unix socket and send them to the p2p network.
    let _tx_from_client_to_world = ingest::<Transaction>(&network, "/var/run/oe4/net.ipc");

    // forward all produced messages of type Transaction arriving from the p2p network
    // to a unix socket for consumption by local client components
    let _tx_from_world_to_client = expose::<Transaction>(&network, "/var/run/oe4/net.ipc");

    let net1 = network.clone();
    tokio::spawn(async move {
      loop {
        println!("devp2p peers: {}", net1.num_peers());
        tokio::time::sleep(Duration::from_secs(3)).await;
      }
    });

    let net2 = network.clone();
    tokio::spawn(async move {
      while let Ok(tx) = receive::<Transaction>(&net2).await {
        println!("received transaction from devp2p: {:?}", tx);
        //send(&tx_from_world_to_client, tx).await; // continue here tomorrow
      }
    });

    let net3 = network.clone();
    tokio::spawn(async move {
      tokio::time::sleep(Duration::from_secs(6)).await;
      println!("network shutdown requested");
      net3.abort().await.unwrap();
    });
  }

  network.await;
  Ok(())
}
