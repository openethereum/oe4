// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

pub mod agent;
pub mod nodes;
pub mod buffers;

pub use nodes::*;
pub use buffers::*;
pub use agent::Agent;

pub use async_trait::async_trait;

#[cfg(test)]
mod tests {
  use serde::{Deserialize, Serialize};

  #[test]
  fn mini_agent_system_sanity() {
    
    #[derive(Clone, Serialize, Deserialize)]
    struct Transaction { 
      id: u64,
      gas: u64,
    }

    #[derive(Clone, Serialize, Deserialize)]
    struct Block {
      id: u64,
      txs: Vec<Transaction>
    }
  }
}