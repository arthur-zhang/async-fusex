//! Implementation of FUSE library

#[allow(clippy::tests_outside_test_module)]
mod abi_marker;
mod context;
mod de;

pub mod file_system;

// ioctl_read!() macro involves inter arithmetic
#[allow(clippy::arithmetic_side_effects)]
pub mod channel;
pub mod fuse_reply;
pub mod fuse_request;
pub mod mount;
// ioctl_read!() macro involves inter arithmetic
mod error;
mod fs_util;
#[allow(clippy::arithmetic_side_effects)]
pub mod protocol;
pub mod session;
mod util;

mod tests {
    use crate::file_system::FileSystem;
    use crate::fs_util::{CreateParam, FileLockParam, INum, RenameParam, SetAttrParam};
    use crate::fuse_reply::{
        ReplyAttr, ReplyBMap, ReplyCreate, ReplyData, ReplyDirectory, ReplyEmpty, ReplyEntry,
        ReplyLock, ReplyOpen, ReplyStatFs, ReplyWrite, ReplyXAttr,
    };
    use crate::fuse_request::Request;
    use crate::mount;
    use crate::session::{Session, new_session};
    use async_trait::async_trait;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use tokio_util::sync::CancellationToken;
    use tracing::{Level, debug};
    use tracing_subscriber::Layer;
    use tracing_subscriber::registry::LookupSpan;
    use tracing_subscriber::util::SubscriberInitExt;

    pub struct Foo {}

    #[async_trait]
    impl FileSystem for Foo {
        async fn init(&self, req: &Request<'_>) -> nix::Result<()> {
            println!("init>>>>");
            return Ok(());
        }

        async fn destroy(&self, req: &Request<'_>) {
            todo!()
        }

        async fn interrupt(&self, req: &Request<'_>, unique: u64) {
            todo!()
        }

        async fn lookup(
            &self,
            req: &Request<'_>,
            parent: INum,
            name: &str,
            reply: ReplyEntry<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn forget(&self, req: &Request<'_>, nlookup: u64) {
            todo!()
        }

        async fn getattr(&self, req: &Request<'_>, reply: ReplyAttr<'_>) -> nix::Result<usize> {
            debug!("getattr {:?}", req);
            let ino = req.nodeid();
            debug!("get attr ino {}", ino);
            return Ok(0);
        }

        async fn setattr(
            &self,
            req: &Request<'_>,
            param: SetAttrParam,
            reply: ReplyAttr<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn readlink(&self, req: &Request<'_>, reply: ReplyData<'_>) -> nix::Result<usize> {
            todo!()
        }

        async fn mknod(
            &self,
            req: &Request<'_>,
            param: CreateParam,
            reply: ReplyEntry<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn mkdir(
            &self,
            req: &Request<'_>,
            parent: INum,
            name: &str,
            mode: u32,
            reply: ReplyEntry<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn unlink(
            &self,
            req: &Request<'_>,
            parent: INum,
            name: &str,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn rmdir(
            &self,
            req: &Request<'_>,
            parent: INum,
            name: &str,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn symlink(
            &self,
            req: &Request<'_>,
            parent: INum,
            name: &str,
            target_path: &Path,
            reply: ReplyEntry<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn rename(
            &self,
            req: &Request<'_>,
            param: RenameParam,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn link(
            &self,
            _req: &Request<'_>,
            _newparent: u64,
            _newname: &str,
            reply: ReplyEntry<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn open(
            &self,
            req: &Request<'_>,
            flags: u32,
            reply: ReplyOpen<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn read(
            &self,
            req: &Request<'_>,
            fh: u64,
            offset: i64,
            size: u32,
            reply: ReplyData<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn write(
            &self,
            req: &Request<'_>,
            fh: u64,
            offset: i64,
            data: Vec<u8>,
            flags: u32,
            reply: ReplyWrite<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn flush(
            &self,
            req: &Request<'_>,
            fh: u64,
            lock_owner: u64,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn release(
            &self,
            req: &Request<'_>,
            fh: u64,
            flags: u32,
            lock_owner: u64,
            flush: bool,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn fsync(
            &self,
            req: &Request<'_>,
            fh: u64,
            datasync: bool,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn opendir(
            &self,
            req: &Request<'_>,
            flags: u32,
            reply: ReplyOpen<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn readdir(
            &self,
            req: &Request<'_>,
            fh: u64,
            offset: i64,
            reply: ReplyDirectory<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn releasedir(
            &self,
            req: &Request<'_>,
            fh: u64,
            flags: u32,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn fsyncdir(
            &self,
            req: &Request<'_>,
            fh: u64,
            datasync: bool,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn statfs(&self, req: &Request<'_>, reply: ReplyStatFs<'_>) -> nix::Result<usize> {
            todo!()
        }

        async fn setxattr(
            &self,
            _req: &Request<'_>,
            _name: &str,
            _value: &[u8],
            _flags: u32,
            _position: u32,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn getxattr(
            &self,
            _req: &Request<'_>,
            _name: &str,
            _size: u32,
            reply: ReplyXAttr<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn listxattr(
            &self,
            _req: &Request<'_>,
            _size: u32,
            reply: ReplyXAttr<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn removexattr(
            &self,
            _req: &Request<'_>,
            _name: &str,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn access(
            &self,
            _req: &Request<'_>,
            _mask: u32,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn create(
            &self,
            _req: &Request<'_>,
            _parent: u64,
            _name: &str,
            _mode: u32,
            _flags: u32,
            reply: ReplyCreate<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn getlk(
            &self,
            _req: &Request<'_>,
            _lk_param: FileLockParam,
            reply: ReplyLock<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn setlk(
            &self,
            _req: &Request<'_>,
            _lk_param: FileLockParam,
            _sleep: bool,
            reply: ReplyEmpty<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }

        async fn bmap(
            &self,
            _req: &Request<'_>,
            _blocksize: u32,
            _idx: u64,
            reply: ReplyBMap<'_>,
        ) -> nix::Result<usize> {
            todo!()
        }
    }
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .init();

        let mount_path = PathBuf::from_str("/dev/foo")?;

        let _ = mount::umount(&mount_path).await;
        // let mount_path = Path::new(mount_point);
        assert!(
            mount_path.is_dir(),
            "the input mount path={mount_path:?} is not a directory"
        );
        let fs = Foo {};
        let session = new_session(&mount_path, fs).await?;

        let cancel_token = CancellationToken::new();
        session.run(cancel_token).await?;
        Ok(())
    }
}
