// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::{Source, Target};
use async_std::task::{self, JoinHandle};
use serde::{de::DeserializeOwned, Serialize};

pub struct IpcOutputNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  _target: &'a dyn Source<T>,
  _worker: JoinHandle<()>,
}

impl<'a, T> IpcOutputNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  pub fn new(target: &'a dyn Source<T>) -> Self {
    IpcOutputNode {
      _target: target,
      _worker: task::spawn(async {}),
    }
  }
}

pub struct IpcInputNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  _target: &'a dyn Target<T>,
  _worker: JoinHandle<()>,
}

impl<'a, T> IpcInputNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  pub fn new(target: &'a dyn Target<T>) -> Self {
    IpcInputNode {
      _target: target,
      _worker: task::spawn(async {}),
    }
  }
}

#[cfg(test)]
mod tests {
  use async_std::os::unix::net::UnixStream;
  use futures_await_test::async_test;

  #[async_test]
  async fn unix_socket() -> Result<(), std::io::Error> {
    let mut _stream = UnixStream::connect("/tmp/socket").await?;

    Ok(())
  }
}
