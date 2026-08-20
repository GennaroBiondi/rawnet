use crate::socket::{ArpSocket, LocalSocket};
use std::io;
use std::os::fd::{AsFd, AsRawFd};
use thiserror::Error;

/// Error type for sending data using a socket
#[derive(Error, Debug)]
pub enum SocketSendError {
    /// A General error
    #[error("{0}")]
    General(#[from] std::io::Error),
}

/// Error type for receiving data using a socket
#[derive(Error, Debug)]
pub enum SocketReceiveError {
    /// A General error
    #[error("{0}")]
    General(#[from] std::io::Error),
}

/// Error type for the connection of a socket
#[derive(Error, Debug)]
pub enum SocketConnectError {
    /// A General error
    #[error("{0}")]
    General(#[from] std::io::Error),

    /// The socket's address is not valid unicode
    #[error("Socket's address is not valid Unicode!")]
    NotUnicode,

    /// The socket's address is too long
    #[error("Socket's address is too long!")]
    TooLong,
}

/// Error type regarding the creation of Sockets.
#[derive(Error, Debug)]
pub enum SocketCreationError {
    /// A General error
    #[error("{0}")]
    General(#[from] std::io::Error),

    /// Failed creating a socket, caused by not having enough permissions
    ///
    /// usually this happens when creating an ARP socket.
    #[error("Not enough permissions")]
    NoPermission,
}

/// Common trait shared between all Socket Types.
pub trait Socket: AsFd {
    /// The type of the Address.
    type Address;

    /// Connect to the given address to initiate packet sharing.
    fn connect(&self, address: &Self::Address) -> Result<(), SocketConnectError>;

    /// Send octets using the socket.
    fn send(&self, buf: &[u8]) -> Result<usize, SocketSendError> {
        let res = unsafe {
            libc::send(
                self.as_fd().as_raw_fd(),
                buf.as_ptr() as *const _,
                buf.len(),
                0,
            )
        };

        if res < 0 {
            use io::Error;
            let error = Error::last_os_error();
            Err(SocketSendError::General(error))
        } else {
            Ok(res as usize)
        }
    }

    /// Receive octets using the socket.
    fn receive(&self, buf: &mut [u8]) -> Result<usize, SocketReceiveError> {
        let res = unsafe {
            libc::recv(
                self.as_fd().as_raw_fd(),
                buf.as_ptr() as *mut _,
                buf.len(),
                0,
            )
        };

        if res < 0 {
            use io::Error;
            let error = Error::last_os_error();
            Err(SocketReceiveError::General(error))
        } else {
            Ok(res as usize)
        }
    }
}

/// A Generalization of a Socket.
///
/// If working with sockets in OSI Layer 4, see [`std::net`].
#[derive(Debug)]
pub enum SocketKind {
    /// ARP Socket.
    Arp(ArpSocket),

    /// Local Socket.
    Local(LocalSocket),
}

impl AsFd for SocketKind {
    fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
        match self {
            Self::Local(x) => x.as_fd(),
            Self::Arp(x) => x.as_fd(),
        }
    }
}

// impl Socket for SocketKind {
//     fn send(&self, buf: &[u8]) -> Result<usize, SocketSendError> {
//         match self {
//             Self::Local(x) => x.send(buf),
//             Self::Arp(x) => x.send(buf),
//         }
//     }
//
//     fn receive(&self, buf: &mut [u8]) -> Result<usize, SocketReceiveError> {
//         match self {
//             Self::Local(x) => x.receive(buf),
//             Self::Arp(x) => x.receive(buf),
//         }
//     }
// }
