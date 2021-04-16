// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

mod config;

pub use config::Config;

use ethereum::{Block, Transaction};
use runtime::{
  agent::{Repeat, Runloop},
  async_trait, Message, Result, Source, Target, UnboundedBuffer,
};
use std::{sync::Arc, time::Duration};

/// Implements ethereum devp2p networking
/// It uses the sentry Rust devp2p implementation from
/// https://github.com/rust-ethereum/sentry/
///
/// This type is exposed as an async target, it allows polling
/// for new work through [receive()].
#[derive(Clone)]
pub struct NetworkInterface {
  localnode: Arc<discv4::Node>,

  /// produced by other peer nodes and usually go to the tx pool
  /// or some other monitoring endpoint
  in_txs: Arc<UnboundedBuffer<Transaction>>,

  /// produced by the local node and published to other peer nodes,
  /// those are usually created by invoking the current node RPC interface
  /// with the intention to have a transaction added to the next block.
  out_txs: Arc<UnboundedBuffer<Transaction>>,

  /// blocks produced by miners by other peers
  in_blocks: Arc<UnboundedBuffer<Block>>,

  /// blocks mined by the current node and meant for the rest of the
  /// network to append it to the blockchain.
  out_blocks: Arc<UnboundedBuffer<Block>>,
}

/// lifetime management
impl NetworkInterface {
  pub async fn new(config: Config) -> std::result::Result<Self, Box<dyn std::error::Error>> {
    Ok(NetworkInterface {
      localnode: discv4::Node::new(
        config.local_addr,
        secp256k1::SecretKey::from_slice(&config.secret_key)?,
        config.boot_nodes,
        None,
        true,
        config.local_port,
      )
      .await?,
      in_txs: Arc::new(UnboundedBuffer::new()),
      out_txs: Arc::new(UnboundedBuffer::new()),
      in_blocks: Arc::new(UnboundedBuffer::new()),
      out_blocks: Arc::new(UnboundedBuffer::new()),
    })
  }
}

#[async_trait]
impl Runloop for NetworkInterface {
  async fn step(&self) -> Repeat {
    // simulate txs coming from the network for now
    runtime::send(&*self.in_txs, Transaction::default()).await;
    Repeat::After(Duration::from_secs(1))
  }
}

/// Public Control API
impl NetworkInterface {
  /// Returns the number of discovered devp2p peers
  pub fn num_peers(&self) -> usize {
    self.localnode.num_nodes()
  }
}

#[async_trait]
impl Source<Transaction> for NetworkInterface {
  fn try_consume(&self) -> Option<Message<Transaction>> {
    self.in_txs.try_consume()
  }

  async fn consume(&self) -> Result<Message<Transaction>> {
    self.in_txs.consume().await
  }
}

#[async_trait]
impl Target<Transaction> for NetworkInterface {
  async fn accept(&self, message: Message<Transaction>) -> runtime::MessageStatus {
    self.out_txs.accept(message).await
  }
}

#[async_trait]
impl Target<Block> for NetworkInterface {
  async fn accept(&self, message: Message<Block>) -> runtime::MessageStatus {
    self.out_blocks.accept(message).await
  }
}

#[async_trait]
impl Source<Block> for NetworkInterface {
  fn try_consume(&self) -> Option<Message<Block>> {
    self.in_blocks.try_consume()
  }

  async fn consume(&self) -> Result<Message<Block>> {
    self.in_blocks.consume().await
  }
}
