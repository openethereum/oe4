// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

mod local;
mod remote;

use async_std::task::{self, JoinHandle};
use std::{
  future::Future,
  ops::Deref,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
  },
  task::{Poll, Waker},
};

pub trait Runloop: Send + Sync {
  fn step(self: Arc<Self>);
}

pub struct Agent<Impl>
where
  Impl: Runloop,
{
  inner: Arc<Impl>,
  worker: RwLock<Option<JoinHandle<()>>>,
  aborted: AtomicBool,
  waker: RwLock<Option<Waker>>,
}

impl<Impl> Agent<Impl>
where
  Impl: Runloop,
{
  pub async fn new(instance: Impl) -> Result<Self, Box<dyn std::error::Error>> {
    let inner = Arc::new(instance);
    Ok(Agent {
      inner: inner.clone(),
      worker: RwLock::new(Some(task::spawn(async move {
        inner.clone();
      }))),
      aborted: AtomicBool::new(false),
      waker: RwLock::new(None),
    })
  }

  pub async fn abort(self) -> std::result::Result<(), Box<dyn std::error::Error>> {
    if !self.aborted.load(Ordering::Relaxed) {
      if let Some(worker) = self.worker.write().unwrap().take() {
        worker.cancel().await;
      }
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

impl<Impl> Drop for Agent<Impl>
where
  Impl: Runloop,
{
  fn drop(&mut self) {
    async_std::task::block_on(self);
  }
}
