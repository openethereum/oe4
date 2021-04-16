// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

mod local;
mod remote;

use async_trait::async_trait;
use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  time::Duration,
};
use tokio::{
  task::{self, JoinHandle},
  time,
};

/// Specifies the amount of time the agent runtime needs to wait
/// before invoking the next step of the runloop
pub enum Repeat {
  Auto,
  Never,
  After(Duration),
}

#[async_trait]
pub trait Runloop: Send + Sync + Sized + Clone + 'static {
  /// Gets invoked in a loop for the duration of the lifetime of the
  /// agent until it is aborted or returns
  async fn step(&self) -> Repeat {
    Repeat::Never
  }

  /// Optional setup code that is executed once before the first step
  /// of the runloop is invoked
  async fn setup(&self) {}

  /// Optional cleanup code that runs once when an abort is requested.
  async fn teardown(&self) {}
}

#[derive(Clone)]
pub struct Agent<Impl>
where
  Impl: Runloop,
{
  inner: Arc<Impl>,
  worker: Arc<JoinHandle<()>>,
  aborted: Arc<AtomicBool>,
}

impl<Impl> Agent<Impl>
where
  Impl: Runloop,
{
  pub async fn new(instance: Impl) -> Result<Self, Box<dyn std::error::Error>> {
    let inner = Arc::new(instance);
    let aborted = Arc::new(AtomicBool::new(false));

    let inner_worker = inner.clone();
    let aborted_worker = aborted.clone();
    let worker = Arc::new(task::spawn(async move {
      inner_worker.setup().await;
      while !aborted_worker.load(Ordering::SeqCst) {
        match inner_worker.step().await {
          Repeat::Never => break,
          Repeat::Auto => task::yield_now().await,
          Repeat::After(duration) => time::sleep(duration).await,
        }
      }
    }));

    Ok(Agent {
      inner: inner.clone(),
      worker: worker,
      aborted: aborted,
    })
  }

  pub async fn abort(self) {
    if !self.aborted.load(Ordering::Relaxed) {
      self.inner.teardown().await;
      self.aborted.store(true, Ordering::SeqCst);
      self.worker.abort();
    }
  }
}

impl<Impl> std::ops::Deref for Agent<Impl>
where
  Impl: Runloop + Send + Sync,
{
  type Target = Impl;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<Impl> std::future::Future for Agent<Impl>
where
  Impl: Runloop,
{
  type Output = ();

  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    match self.aborted.load(Ordering::Relaxed) {
      true => std::task::Poll::Ready(()),
      false => {
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
      }
    }
  }
}
