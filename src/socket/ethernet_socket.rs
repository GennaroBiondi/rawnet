use std::{
    io::{Error, ErrorKind},
    os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd},
};

use crate::{
    ethernet_address::EthernetAddress,
    socket::{Socket, SocketConnectError, SocketCreationError},
};

/// Struct for working with Ethernet Frames
///
/// Normally the kernel would automatically send Ethernet frames for you
/// when using sockets, but if using the `AF_PACKET` domain and `SOCK_DGRAM` type
/// you can send your own ethernet frames manually.
#[derive(Debug)]
pub struct EthernetSocket {
    fd: OwnedFd,
}

impl EthernetSocket {
    pub fn new() -> Result<EthernetSocket, SocketCreationError> {
        let raw_fd = unsafe {
            use libc::{AF_PACKET, SOCK_DGRAM, socket};

            let fd = socket(AF_PACKET, SOCK_DGRAM, 0);

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

        Ok(Self { fd })
    }
}

impl AsFd for EthernetSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}
