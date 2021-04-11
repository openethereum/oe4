// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::{Source, Target};
use async_std::os::unix::net::UnixStream;
use serde::{de::DeserializeOwned, Serialize};
use std::io;

pub struct IpcOutputNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  target: &'a dyn Source<T>,
  stream: UnixStream,
}

impl<'a, T> IpcOutputNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  pub async fn new(
    target: &'a dyn Source<T>,
    path: &str,
  ) -> Result<IpcOutputNode<'a, T>, std::io::Error> {
    Ok(IpcOutputNode {
      target: target,
      stream: UnixStream::connect(path).await?,
    })
  }
}

pub struct IpcInputNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  target: &'a dyn Target<T>,
  stream: UnixStream,
}

impl<'a, T> IpcInputNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  pub async fn new(
    target: &'a dyn Target<T>,
    path: &str,
  ) -> Result<IpcInputNode<'a, T>, io::Error> {
    Ok(IpcInputNode {
      target: target,
      stream: UnixStream::connect(path).await?,
    })
  }
}

pub async fn expose<'a, T>(
  target: &'a dyn Source<T>,
  path: &str,
) -> Result<IpcOutputNode<'a, T>, io::Error>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  IpcOutputNode::new(target, path).await
}

pub async fn ingest<'a, T>(
  target: &'a dyn Target<T>,
  path: &str,
) -> Result<IpcInputNode<'a, T>, io::Error>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  IpcInputNode::new(target, path).await
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
