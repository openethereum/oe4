// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

mod config;

pub use config::Config;

use ethereum::{Block, Transaction};
use runtime::{agent::Runloop, async_trait, Message, Result, Source, Target, UnboundedBuffer};
use std::{
  future::Future,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
  },
  task::{Poll, Waker},
  time::Duration,
};
use tokio::{task::JoinHandle, time::sleep};

/// Implements ethereum devp2p networking
/// It uses the sentry Rust devp2p implementation from
/// https://github.com/rust-ethereum/sentry/
///
/// This type is exposed as an async target, it allows polling
/// for new work through [receive()].
#[derive(Clone)]
pub struct NetworkInterface {
  inner: Arc<NetworkInterfaceInner>,
  worker: Arc<JoinHandle<()>>,
  aborted: Arc<AtomicBool>,
  waker: Arc<RwLock<Option<Waker>>>,
}

struct NetworkInterfaceInner {
  localnode: Arc<discv4::Node>,

  /// produced by other peer nodes and usually go to the tx pool
  /// or some other monitoring endpoint
  in_txs: UnboundedBuffer<Transaction>,

  /// produced by the local node and published to other peer nodes,
  /// those are usually created by invoking the current node RPC interface
  /// with the intention to have a transaction added to the next block.
  out_txs: UnboundedBuffer<Transaction>,

  /// blocks produced by miners by other peers
  in_blocks: UnboundedBuffer<Block>,

  /// blocks mined by the current node and meant for the rest of the
  /// network to append it to the blockchain.
  out_blocks: UnboundedBuffer<Block>,
}

/// lifetime management
impl NetworkInterface {
  pub async fn new(config: Config) -> std::result::Result<Self, Box<dyn std::error::Error>> {
    let inner = Arc::new(NetworkInterfaceInner {
      localnode: discv4::Node::new(
        config.local_addr,
        secp256k1::SecretKey::from_slice(&config.secret_key)?,
        config.boot_nodes,
        None,
        true,
        config.local_port,
      )
      .await?,
      in_txs: UnboundedBuffer::new(),
      out_txs: UnboundedBuffer::new(),
      in_blocks: UnboundedBuffer::new(),
      out_blocks: UnboundedBuffer::new(),
    });

    Ok(NetworkInterface {
      inner: inner.clone(),
      worker: Arc::new(tokio::spawn(NetworkInterface::runloop(inner.clone()))),
      aborted: Arc::new(AtomicBool::new(false)),
      waker: Arc::new(RwLock::new(None)),
    })
  }

  pub async fn abort(mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
    if !self.aborted.load(Ordering::Relaxed) {
      self.worker.abort();
      if let Some(worker) = Arc::get_mut(&mut self.worker) {
        worker.await?;
      }
      if let Some(ref waker) = *self.waker.read().unwrap() {
        waker.wake_by_ref();
      }
      self.aborted.store(true, Ordering::SeqCst);
    }
    Ok(())
  }

  // simulate txs coming from the network for now
  async fn runloop(network_interface: Arc<NetworkInterfaceInner>) {
    loop {
      sleep(Duration::from_secs(3)).await;
      runtime::send(&network_interface.in_txs, Transaction::default()).await;
    }
  }
}

#[async_trait]
impl Runloop for NetworkInterface {
  async fn step(self: Arc<Self>) {
    todo!()
  }

  async fn teardown() -> std::result::Result<(), Box<dyn std::error::Error>> {
    todo!()
  }
}

impl Future for NetworkInterface {
  type Output = ();

  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    match self.aborted.load(Ordering::Relaxed) {
      true => Poll::Ready(()),
      false => {
        *self.get_mut().waker.write().unwrap() = Some(cx.waker().clone());
        Poll::Pending
      }
    }
  }
}

impl Drop for NetworkInterface {
  fn drop(&mut self) {
    futures::executor::block_on(self)
  }
}

/// Public Control API
impl NetworkInterface {
  /// Returns the number of discovered devp2p peers
  pub fn num_peers(&self) -> usize {
    self.inner.localnode.num_nodes()
  }
}

#[async_trait]
impl Source<Transaction> for NetworkInterface {
  fn try_consume(&self) -> Option<Message<Transaction>> {
    self.inner.in_txs.try_consume()
  }

  async fn consume(&self) -> Result<Message<Transaction>> {
    self.inner.in_txs.consume().await
  }
}

#[async_trait]
impl Target<Transaction> for NetworkInterface {
  async fn accept(&self, message: Message<Transaction>) -> runtime::MessageStatus {
    self.inner.out_txs.accept(message).await
  }
}

#[async_trait]
impl Target<Block> for NetworkInterface {
  async fn accept(&self, message: Message<Block>) -> runtime::MessageStatus {
    self.inner.out_blocks.accept(message).await
  }
}

#[async_trait]
impl Source<Block> for NetworkInterface {
  fn try_consume(&self) -> Option<Message<Block>> {
    self.inner.in_blocks.try_consume()
  }

  async fn consume(&self) -> Result<Message<Block>> {
    self.inner.in_blocks.consume().await
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
