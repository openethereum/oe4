// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

pub mod broadcast;
pub mod proxy;

use proxy::{IpcInputNode, IpcOutputNode};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Source, Target};

pub fn expose<'a, T>(target: &'a dyn Source<T>, _path: &str) -> IpcOutputNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  IpcOutputNode::new(target)
}

pub fn ingest<'a, T>(_target: &'a dyn Target<T>, _path: &str) -> IpcInputNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  todo!();
}
