use std::io;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd};
use std::path::PathBuf;
use thiserror::Error;

/// A general error type for Sockets.
#[derive(Error, Debug)]
pub enum SocketError {
    /// Failed creating a Socket
    #[error("Failed to create socket: {0}")]
    Creation(#[from] SocketCreationError),

    /// Failed to send data using a Socket
    #[error("Failed to send data using socket: {0}")]
    Send(#[from] SocketSendError),

    /// Failed to receive data using a Socket
    #[error("Failed to receive data using socket: {0}")]
    Receive(#[from] SocketReceiveError),
}

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

/// error type regarding the creation of Sockets
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

pub trait Socket: AsFd {
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

#[derive(Debug)]
pub struct ArpSocket {
    fd: OwnedFd,
}

impl AsFd for ArpSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

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
                use io::{Error, ErrorKind};

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

impl Socket for ArpSocket {}

/// A Generalization of a Socket.
#[derive(Debug)]
pub enum SocketKind {
    Arp(ArpSocket),
    Local(LocalSocket),
}
