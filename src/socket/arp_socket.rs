use std::{
    io::{Error, ErrorKind},
    net::Ipv4Addr,
    os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd},
};

use thiserror::Error;

use crate::{
    MacAddress,
    socket::{Socket, SocketConnectError, SocketCreationError},
};

/// Struct for working with the ARP Protocol.
#[derive(Debug)]
pub struct ArpSocket {
    fd: OwnedFd,
}

impl ArpSocket {
    /// Construct a new ArpSocket.
    pub fn new() -> Result<Self, SocketCreationError> {
        let raw_fd = unsafe {
            use libc::{AF_PACKET, SOCK_RAW, socket};

            let fd = socket(AF_PACKET, SOCK_RAW, 0);

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

/// Struct to easily build ARP packets.
///
/// This struct aligns with the structure of an ARP packet.
/// The purpose of the struct is to modify its fields and
/// then try to transform it into a vector of bytes to send.
///
/// # Examples
///
/// ```no_run
/// use rawnet::{MacAddress, socket::{ArpSocket, Socket}};
/// use std::net::Ipv4Addr;
///
/// let arp_socket = ArpSocket::new().unwrap();
/// let arp_header = ArpHeader::request(MacAddress::ZERO, Ipv4Addr::LOCALHOST, Ipv4Addr::UNSPECIFIED);
/// let arp_packet = (&arp_header).into();
/// arp_socket.send(&arp_packet).unwrap();
/// ```
#[derive(Debug)]
#[repr(C, packed)]
pub struct ArpHeader {
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

/// Error type for converting bytes into an ArpHeader
#[derive(Error, Debug)]
pub enum ArpHeaderConvertError {
    #[error("Input has {0} bytes instead of 28")]
    InvalidLength(usize),
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

impl From<&ArpHeader> for Vec<u8> {
    fn from(value: &ArpHeader) -> Self {
        let mut bytes = Vec::with_capacity(28);
        bytes.extend_from_slice(&value.hardware_type.to_be_bytes());
        bytes.extend_from_slice(&value.protocol_type.to_be_bytes());
        bytes.push(value.hw_addr_len);
        bytes.push(value.proto_addr_len);
        bytes.extend_from_slice(&value.operation.to_be_bytes());
        bytes.extend_from_slice(&value.sender_mac.into_array());
        bytes.extend_from_slice(&value.sender_ip);
        bytes.extend_from_slice(&value.target_mac.into_array());
        bytes.extend_from_slice(&value.target_ip);
        bytes
    }
}

impl TryFrom<&[u8]> for ArpHeader {
    type Error = ArpHeaderConvertError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let bytes_amount = value.len();
        if bytes_amount != 28 {
            return Err(ArpHeaderConvertError::InvalidLength(bytes_amount));
        }

        Ok(Self {
            hardware_type: u16::from_be_bytes([value[0], value[1]]),
            protocol_type: u16::from_be_bytes([value[2], value[3]]),
            hw_addr_len: value[4],
            proto_addr_len: value[5],
            operation: u16::from_be_bytes([value[6], value[7]]),
            sender_mac: MacAddress::from([
                value[8], value[9], value[10], value[11], value[12], value[13],
            ]),
            sender_ip: [value[14], value[15], value[16], value[17]],
            target_mac: MacAddress::from([
                value[18], value[19], value[20], value[21], value[22], value[23],
            ]),
            target_ip: [value[24], value[25], value[26], value[27]],
        })
    }
}

impl ArpHeader {
    /// Construct an ArpHeader by only providing the sender's MacAddress, IP, and the target's IP.
    ///
    /// This is the most common use of the ARP protocol, finding the MacAddress of a device by
    /// knowing its IP address
    pub const fn request(sender_mac: MacAddress, sender_ip: Ipv4Addr, target_ip: Ipv4Addr) -> Self {
        Self {
            hardware_type: 1,
            protocol_type: 0x0800,
            hw_addr_len: 6,
            proto_addr_len: 4,
            operation: 1,
            sender_mac,
            sender_ip: sender_ip.octets(),
            target_mac: MacAddress::BROADCAST,
            target_ip: target_ip.octets(),
        }
    }

    /// Construct an ARP reply header.
    ///
    /// This is used when responding to an ARP request. The sender is the device
    /// replying with its own MAC address, and the target is the original requester.
    pub const fn reply(
        sender_mac: MacAddress,
        sender_ip: Ipv4Addr,
        target_mac: MacAddress,
        target_ip: Ipv4Addr,
    ) -> Self {
        Self {
            hardware_type: 1,
            protocol_type: 0x0800,
            hw_addr_len: 6,
            proto_addr_len: 4,
            operation: 2,
            sender_mac,
            sender_ip: sender_ip.octets(),
            target_mac,
            target_ip: target_ip.octets(),
        }
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
