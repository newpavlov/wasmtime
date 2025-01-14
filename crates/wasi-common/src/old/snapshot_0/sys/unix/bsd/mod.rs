pub(crate) mod filetime;
pub(crate) mod hostcalls_impl;
pub(crate) mod osfile;

pub(crate) mod fdentry_impl {
    use crate::old::snapshot_0::{sys::host_impl, Result};
    use std::os::unix::prelude::AsRawFd;

    pub(crate) unsafe fn isatty(fd: &impl AsRawFd) -> Result<bool> {
        let res = libc::isatty(fd.as_raw_fd());
        if res == 1 {
            // isatty() returns 1 if fd is an open file descriptor referring to a terminal...
            Ok(true)
        } else {
            // ... otherwise 0 is returned, and errno is set to indicate the error.
            match nix::errno::Errno::last() {
                nix::errno::Errno::ENOTTY => Ok(false),
                x => Err(host_impl::errno_from_nix(x)),
            }
        }
    }
}

pub(crate) mod host_impl {
    use super::super::host_impl::dirent_filetype_from_host;
    use crate::old::snapshot_0::{wasi, Result};

    pub(crate) const O_RSYNC: nix::fcntl::OFlag = nix::fcntl::OFlag::O_SYNC;

    pub(crate) fn dirent_from_host(
        host_entry: &nix::libc::dirent,
    ) -> Result<wasi::__wasi_dirent_t> {
        let mut entry = unsafe { std::mem::zeroed::<wasi::__wasi_dirent_t>() };
        let d_type = dirent_filetype_from_host(host_entry)?;
        entry.d_ino = host_entry.d_ino;
        entry.d_next = host_entry.d_seekoff;
        entry.d_namlen = u32::from(host_entry.d_namlen);
        entry.d_type = d_type;
        Ok(entry)
    }
}
