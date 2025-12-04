# async-fusex

[![Crates.io](https://img.shields.io/crates/v/async-fusex.svg)](https://crates.io/crates/async-fusex)
[![Documentation](https://docs.rs/async-fusex/badge.svg)](https://docs.rs/async-fusex)
[![License](https://img.shields.io/crates/l/async-fusex.svg)](LICENSE)

An async-friendly Rust library for building FUSE (Filesystem in Userspace) filesystems. Origin code is taken from DatenLord.

## Overview

async-fusex provides a high-level, async/await-based abstraction over the FUSE kernel interface, making it easy to develop custom filesystems that run in userspace. Built on top of Tokio, it enables efficient concurrent handling of filesystem requests.

## Features

- **Async/Await Support**: Built on Tokio for non-blocking, concurrent request handling
- **Full FUSE Protocol Support**: Supports FUSE ABI versions 7.8 through 7.31
- **Complete Filesystem Operations**:
  - File operations: create, read, write, open, release, flush, fsync
  - Directory operations: mkdir, rmdir, opendir, readdir, releasedir
  - Metadata: getattr, setattr, lookup, statfs
  - Links: symlink, hardlink, rename, unlink
  - Extended attributes: setxattr, getxattr, listxattr, removexattr
  - POSIX file locking: getlk, setlk
- **Multi-threaded Request Handling**: Configurable FUSE device reader threads with buffer pooling
- **Graceful Shutdown**: Cancellation token support for clean unmounting

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
async-fusex = "0.1"
```

## Usage

Implement the `FileSystem` trait and create a session:

```rust
use async_fusex::{FileSystem, Session, Request, ReplyEntry, ReplyAttr};
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use std::path::Path;

struct MyFilesystem;

#[async_trait]
impl FileSystem for MyFilesystem {
    async fn init(&self, req: &Request<'_>) -> nix::Result<()> {
        // Initialize your filesystem
        Ok(())
    }

    async fn lookup(
        &self,
        req: &Request<'_>,
        parent: u64,
        name: &str,
        reply: ReplyEntry<'_>,
    ) -> nix::Result<usize> {
        // Handle directory entry lookup
        // ...
    }

    async fn getattr(
        &self,
        req: &Request<'_>,
        ino: u64,
        reply: ReplyAttr<'_>,
    ) -> nix::Result<usize> {
        // Return file attributes
        // ...
    }

    // Implement other operations as needed...
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let fs = MyFilesystem;
    let mountpoint = Path::new("/mnt/myfs");

    let session = async_fusex::new_session(mountpoint, fs).await?;
    let token = CancellationToken::new();

    session.run(token).await?;
    Ok(())
}
```

## Architecture

```
User Code (FileSystem Implementation)
        │
        ▼
FileSystem Trait (async interface)
        │
        ▼
    Session (FUSE session manager)
        │
        ▼
FUSE Device Reader Threads + Tokio Runtime
        │
        ▼
   /dev/fuse (Kernel Interface)
```

## Requirements

- Linux with FUSE support
- Rust 2024 edition

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
