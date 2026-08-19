use std::os::fd::{AsFd, BorrowedFd, OwnedFd};

use crate::socket::Socket;

/// Struct for working with the ARP Protocol
#[derive(Debug)]
pub struct ArpSocket {
    fd: OwnedFd,
}

impl AsFd for ArpSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl Socket for ArpSocket {}
