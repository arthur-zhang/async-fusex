# async-fusex

[![Crates.io](https://img.shields.io/crates/v/async-fusex.svg)](https://crates.io/crates/async-fusex)
[![Documentation](https://docs.rs/async-fusex/badge.svg)](https://docs.rs/async-fusex)
[![License](https://img.shields.io/crates/l/async-fusex.svg)](LICENSE)

An async-friendly Rust library for building FUSE (Filesystem in Userspace) filesystems. Original code is taken from DatenLord project, with many code modified.

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

Implement the `VirtualFs` trait and create a session:

```rust
use async_fusex::{
    VirtualFs, FuseFs, DirEntry, FileType,
    fs_util::{FileAttr, INum, ROOT_ID},
    error::AsyncFusexResult,
};
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use std::sync::Arc;
use std::time::Duration;
use std::path::Path;

struct MyFilesystem;

#[async_trait]
impl VirtualFs for MyFilesystem {
    async fn lookup(
        &self,
        uid: u32,
        gid: u32,
        parent: INum,
        name: &str,
    ) -> AsyncFusexResult<(Duration, FileAttr, u64)> {
        // Look up a directory entry by name and return its attributes
        // ...
    }

    async fn forget(&self, ino: u64, nlookup: u64) {
        // Called when the kernel removes an inode from its cache
    }

    async fn getattr(&self, ino: u64) -> AsyncFusexResult<(Duration, FileAttr)> {
        // Return file attributes for the given inode
        // ...
    }

    async fn readdir(
        &self,
        uid: u32,
        gid: u32,
        ino: u64,
        fh: u64,
        offset: i64,
    ) -> AsyncFusexResult<Vec<DirEntry>> {
        // Return directory entries
        // ...
    }

    // Implement other operations as needed:
    // open, read, write, mkdir, rmdir, unlink, etc.
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let my_fs = Arc::new(MyFilesystem);
    let fuse_fs = FuseFs::new(my_fs);
    let mountpoint = Path::new("/mnt/myfs");

    let session = async_fusex::session::new_session(mountpoint, fuse_fs).await?;
    let token = CancellationToken::new();

    session.run(token).await?;
    Ok(())
}
```

## Architecture

```
User Code (VirtualFs Implementation)
        │
        ▼
VirtualFs Trait (high-level async interface)
        │
        ▼
    FuseFs (adapter)
        │
        ▼
FileSystem Trait (low-level FUSE operations)
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

## get your fuse api version

```
dmesg | grep fuse

[    2.095512] fuse: init (API version 7.39)
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
