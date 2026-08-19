use std::ffi::c_char;
use std::io::{Error, ErrorKind};
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd};
use std::{os::fd::OwnedFd, path::PathBuf};

use crate::socket::{Socket, SocketConnectError, SocketCreationError, SocketError};

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

impl Socket for LocalSocket {
    type Address = PathBuf;

    fn connect(&self, address: &Self::Address) -> Result<(), SocketConnectError> {
        use libc::{AF_UNIX, connect, sockaddr_un};
        use std::mem::size_of;

        let path = address.to_str().ok_or(SocketConnectError::NotUnicode)?;
        let path_bytes = path.as_bytes();

        let mut sock_addr: sockaddr_un = unsafe { std::mem::zeroed() };
        sock_addr.sun_family = AF_UNIX as _;

        for (i, &byte) in path_bytes.iter().enumerate().take(107) {
            sock_addr.sun_path[i] = byte as c_char;
        }
        sock_addr.sun_path[path_bytes.len()] = 0;

        let res = unsafe {
            connect(
                self.fd.as_raw_fd(),
                &sock_addr as *const _ as *const _,
                size_of::<sockaddr_un>() as _,
            )
        };

        if res < 0 {
            Err(SocketConnectError::General(Error::last_os_error()))
        } else {
            Ok(())
        }
    }
}

impl AsFd for LocalSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}
