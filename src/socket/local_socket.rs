use std::io::{Error, ErrorKind};
use std::os::fd::{AsFd, BorrowedFd, FromRawFd};
use std::{os::fd::OwnedFd, path::PathBuf};

use crate::socket::{SocketCreationError, SocketError};

/// Struct for working with UNIX Sockets (also called local sockets)
///
/// a Local Socket is useful when working with IPC.
#[derive(Debug)]
pub struct LocalSocket {
    fd: OwnedFd,
    path: Option<PathBuf>,
}

impl LocalSocket {
    /// Construct a new LocalSocket.
    pub fn new() -> Result<Self, SocketError> {
        let raw_fd = unsafe {
            use libc::{AF_LOCAL, SOCK_STREAM, socket};

            let fd = socket(AF_LOCAL, SOCK_STREAM, 0);

            if fd < 0 {
                let error = Error::last_os_error();
                let error_kind = error.kind();

                match error_kind {
                    ErrorKind::PermissionDenied => Err(SocketCreationError::NoPermission),
                    _ => Err(SocketCreationError::General(error)),
                }
            } else {
                Ok(fd)
            }
        }?;

        let fd = unsafe { OwnedFd::from_raw_fd(raw_fd) };

        Ok(Self { fd, path: None })
    }
}

impl AsFd for LocalSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}
