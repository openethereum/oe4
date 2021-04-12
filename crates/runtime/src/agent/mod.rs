// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

mod local;
mod remote;

use async_std::{
  sync::{Condvar, Mutex},
  task::{self, JoinHandle},
};
use async_trait::async_trait;
use std::{
  error::Error,
  future::Future,
  ops::Deref,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
  },
  task::{Poll, Waker},
};

#[async_trait]
pub trait Runloop: Send + Sync + Sized {
  async fn step(self: Arc<Self>);
  async fn teardown() -> Result<(), Box<dyn Error>>;
}

pub struct Agent<Impl>
where
  Impl: Runloop,
{
  inner: Arc<Impl>,
  worker: JoinHandle<()>,
  aborted: AtomicBool,
  waker: RwLock<Option<Waker>>,
}

impl<Impl> Agent<Impl>
where
  Impl: Runloop,
{
  pub async fn new(instance: Impl) -> Result<Self, Box<dyn std::error::Error>> {
    let var = Arc::new(Condvar::new());
    
    let var_clone = var.clone();
    let inner = Arc::new(instance);

    let worker = task::spawn(async move {
      let second = inner.clone();
      var_clone.notify_one();
      
      // start inner runloop (omitted)
    });

    let lock = Mutex::new(false);
    var.wait(lock.lock().await).await;

    Ok(Agent {
      inner: inner.clone(),
      worker: worker,
      aborted: AtomicBool::new(false),
      waker: RwLock::new(None),
    })
  }

  pub async fn abort(self) -> std::result::Result<(), Box<dyn std::error::Error>> {
    if !self.aborted.load(Ordering::Relaxed) {
      self.worker.cancel().await;
      if let Some(ref waker) = *self.waker.read().unwrap() {
        waker.wake_by_ref();
      }
      self.aborted.store(true, Ordering::SeqCst);
    }
    Ok(())
  }
}

impl<Impl> Deref for Agent<Impl>
where
  Impl: Runloop + Send + Sync,
{
  type Target = Impl;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<Impl> Future for Agent<Impl>
where
  Impl: Runloop,
{
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

// impl<Impl> Drop for Agent<Impl>
// where
//   Impl: Runloop,
// {
//   fn drop(&mut self) {
//     async_std::task::block_on(self);
//   }
// }
