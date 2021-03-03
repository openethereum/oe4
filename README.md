# OpenEthereum 4.0

OpenEthereum 4.0 - Planning &amp; Design Repository

This repository is used to create a blueprint of the system design for the new OE4 edition. At the moment it is used to gather the most important high-level design decisions.

Please browse through individual crates for more specific discussions and/or design decisions:

  - [Core](crates/core/README.md) (fundamental types)
  - [Execution](crates/execution/README.md) (evm)
  - [Networking](crates/networking/README.md) (devp2p, libp2p, json-rpc)
  - [Storage](crates/storage/README.md) (snapshotting, import/export, state, blocks store, pruning, archival, etc.)
  - [Consesnsus](crates/consesnsus/README.md) (PoW, PoS, Miner, etc..)
  - [Transaction Pool](crates/txpool/README.md)
  - [OE](crates/oe/README.md)
