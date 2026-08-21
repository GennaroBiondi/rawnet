use std::{
    io::{Error, ErrorKind},
    net::Ipv4Addr,
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

impl Socket for EthernetSocket {
    type Address = EthernetAddress;

    fn connect(&self, address: &Self::Address) -> Result<(), SocketConnectError> {
        use libc::{AF_PACKET, connect, sockaddr_ll};
        use std::mem::size_of;

        let path = address;
        let path_bytes = path.octets();

        let mut sock_addr: sockaddr_ll = unsafe { std::mem::zeroed() };
        sock_addr.sll_family = AF_PACKET as _;

        for (i, &byte) in path_bytes.iter().enumerate() {
            sock_addr.sll_addr[i] = byte;
        }

        let res = unsafe {
            connect(
                self.fd.as_raw_fd(),
                &sock_addr as *const _ as *const _,
                size_of::<sockaddr_ll>() as _,
            )
        };

        if res < 0 {
            Err(SocketConnectError::General(Error::last_os_error()))
        } else {
            Ok(())
        }
    }
}
