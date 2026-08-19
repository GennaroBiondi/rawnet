use std::io;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// a general error type for Sockets.
#[derive(Error, Debug)]
pub enum SocketError {
    #[error("Failed to create socket: {0}")]
    SocketCreation(#[from] SocketCreationError),
    #[error("Failed to send data using socket: {0}")]
    SocketSend(#[from] SocketSendError),
    #[error("Failed to receive data using socket: {0}")]
    SocketReceive(#[from] SocketReceiveError),
}

#[derive(Error, Debug)]
pub enum SocketSendError {
    #[error("{0}")]
    General(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum SocketReceiveError {
    #[error("{0}")]
    General(#[from] std::io::Error),
}

/// error type regarding the creation of Sockets
#[derive(Error, Debug)]
pub enum SocketCreationError {
    #[error("{0}")]
    General(#[from] std::io::Error),

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

#[derive(Debug)]
pub struct LocalSocket {
    fd: OwnedFd,
    path: Option<PathBuf>,
}

impl LocalSocket {
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

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

impl AsFd for LocalSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl Socket for ArpSocket {}

#[derive(Debug)]
pub enum RawSocket {
    Arp(ArpSocket),
}
