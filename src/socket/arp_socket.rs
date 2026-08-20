use std::{
    io::Error,
    net::Ipv4Addr,
    os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd},
};

use crate::{
    MacAddress,
    socket::{Socket, SocketConnectError},
};

/// Struct for working with the ARP Protocol
#[derive(Debug)]
pub struct ArpSocket {
    fd: OwnedFd,
}

/// Struct to easily build arp requests
#[derive(Debug)]
#[repr(C, packed)]
struct ArpHeader {
    pub hardware_type: u16,
    pub protocol_type: u16,
    pub hw_addr_len: u8,
    pub proto_addr_len: u8,
    pub operation: u16,
    pub sender_mac: MacAddress,
    pub sender_ip: [u8; 4],
    pub target_mac: MacAddress,
    pub target_ip: [u8; 4],
}

impl Default for ArpHeader {
    fn default() -> Self {
        Self {
            hardware_type: 0,
            protocol_type: 0,
            hw_addr_len: 0,
            proto_addr_len: 0,
            operation: 0,
            sender_mac: MacAddress::ZERO,
            sender_ip: Ipv4Addr::UNSPECIFIED.octets(),
            target_mac: MacAddress::ZERO,
            target_ip: Ipv4Addr::UNSPECIFIED.octets(),
        }
    }
}

impl ArpHeader {
    fn as_packet(&self) -> Vec<u8> {
        unsafe {
            std::slice::from_raw_parts(
                self as *const ArpHeader as *const u8,
                std::mem::size_of::<ArpHeader>(),
            )
        }
        .to_vec()
    }
}

impl AsFd for ArpSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl Socket for ArpSocket {
    type Address = Ipv4Addr;

    fn connect(&self, address: &Self::Address) -> Result<(), super::SocketConnectError> {
        use libc::{AF_PACKET, connect, sockaddr_ll};
        use std::mem::size_of;

        let path = address;
        let path_bytes = path.octets();

        let mut sock_addr: sockaddr_ll = unsafe { std::mem::zeroed() };
        sock_addr.sll_family = AF_PACKET as _;

        for (i, &byte) in path_bytes.iter().enumerate() {
            sock_addr.sll_addr[i] = byte as u8;
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
