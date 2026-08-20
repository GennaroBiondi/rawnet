use crate::MacAddress;
use std::net::Ipv4Addr;
use thiserror::Error;

/// Hardware type values as defined by the IANA ARP Hardware Types registry.
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum ArpHardwareType {
    /// Unspecified
    Reserved = 0,
    /// Ethernet (10Mb)
    Ethernet = 1,
    /// Experimental Ethernet (3Mb)
    ExperimentalEthernet = 2,
    /// Amateur Radio AX.25
    AmateurRadioAx25 = 3,
    /// Proteon ProNet Token Ring
    ProteonProNetTokenRing = 4,
    /// Chaos
    Chaos = 5,
    /// IEEE 802 Networks
    Ieee802Networks = 6,
    /// ARCNET
    Arcnet = 7,
    /// Hyperchannel
    Hyperchannel = 8,
    /// Lanstar
    Lanstar = 9,
    /// Autonet Short Address
    AutonetShortAddress = 10,
    /// LocalTalk
    LocalTalk = 11,
    /// LocalNet (IBM PCNet / SYTEK)
    LocalNet = 12,
    /// Ultra link
    UltraLink = 13,
    /// SMDS
    Smds = 14,
    /// Frame Relay
    FrameRelay = 15,
    /// Asynchronous Transmission Mode (ATM)
    Atm = 16,
    /// HDLC
    Hdlc = 17,
    /// Fibre Channel
    FibreChannel = 18,
    /// Serial Line
    SerialLine = 20,
}

/// ARP operation codes as defined by RFC 826.
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum ArpOperationCode {
    /// Unspecified
    Reserved = 0,
    /// ARP request.
    Request = 1,
    /// ARP reply.
    Reply = 2,
    /// Reverse ARP request.
    ReverseRequest = 3,
    /// Reverse ARP reply.
    ReverseReply = 4,
    /// Dynamic RARP request.
    DrarpRequest = 5,
    /// Dynamic RARP reply.
    DrarpReply = 6,
    /// Dynamic RARP error.
    DrarpError = 7,
    /// Inverse ARP request.
    InarpRequest = 8,
    /// Inverse ARP reply.
    InarpReply = 9,
}

/// Protocol type values as defined by the IANA EtherType registry.
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum ArpProtocolType {
    /// Unspecified
    Reserved = 0,
    /// Internet Protocol version 4 (IPv4)
    Ipv4 = 0x0800,
    /// Address Resolution Protocol (ARP)
    Arp = 0x0806,
    /// Wake-on-LAN
    WakeOnLan = 0x0842,
    /// Reverse Address Resolution Protocol (RARP)
    Rarp = 0x8035,
    /// AppleTalk
    AppleTalk = 0x809B,
    /// AppleTalk Address Resolution Protocol (AARP)
    Aarp = 0x80F3,
    /// Internet Protocol version 6 (IPv6)
    Ipv6 = 0x86DD,
}

/// Error type for converting a u16 into an ARP enum.
#[derive(Error, Debug)]
pub enum ArpEnumConvertError {
    /// The u16 value does not correspond to a known variant.
    #[error("Unknown value: {0}")]
    UnknownValue(u16),
}

impl TryFrom<u16> for ArpHardwareType {
    type Error = ArpEnumConvertError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Reserved),
            1 => Ok(Self::Ethernet),
            2 => Ok(Self::ExperimentalEthernet),
            3 => Ok(Self::AmateurRadioAx25),
            4 => Ok(Self::ProteonProNetTokenRing),
            5 => Ok(Self::Chaos),
            6 => Ok(Self::Ieee802Networks),
            7 => Ok(Self::Arcnet),
            8 => Ok(Self::Hyperchannel),
            9 => Ok(Self::Lanstar),
            10 => Ok(Self::AutonetShortAddress),
            11 => Ok(Self::LocalTalk),
            12 => Ok(Self::LocalNet),
            13 => Ok(Self::UltraLink),
            14 => Ok(Self::Smds),
            15 => Ok(Self::FrameRelay),
            16 => Ok(Self::Atm),
            17 => Ok(Self::Hdlc),
            18 => Ok(Self::FibreChannel),
            20 => Ok(Self::SerialLine),
            _ => Err(ArpEnumConvertError::UnknownValue(value)),
        }
    }
}

impl TryFrom<u16> for ArpOperationCode {
    type Error = ArpEnumConvertError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Reserved),
            1 => Ok(Self::Request),
            2 => Ok(Self::Reply),
            3 => Ok(Self::ReverseRequest),
            4 => Ok(Self::ReverseReply),
            5 => Ok(Self::DrarpRequest),
            6 => Ok(Self::DrarpReply),
            7 => Ok(Self::DrarpError),
            8 => Ok(Self::InarpRequest),
            9 => Ok(Self::InarpReply),
            _ => Err(ArpEnumConvertError::UnknownValue(value)),
        }
    }
}

impl TryFrom<u16> for ArpProtocolType {
    type Error = ArpEnumConvertError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Reserved),
            0x0800 => Ok(Self::Ipv4),
            0x0806 => Ok(Self::Arp),
            0x0842 => Ok(Self::WakeOnLan),
            0x8035 => Ok(Self::Rarp),
            0x809B => Ok(Self::AppleTalk),
            0x80F3 => Ok(Self::Aarp),
            0x86DD => Ok(Self::Ipv6),
            _ => Err(ArpEnumConvertError::UnknownValue(value)),
        }
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
/// use rawnet::{MacAddress, packet::ArpPacket, socket::{ArpSocket, Socket}};
/// use std::net::Ipv4Addr;
///
/// let arp_socket = ArpSocket::new().unwrap();
/// let arp_packet = ArpPacket::request(MacAddress::ZERO, Ipv4Addr::LOCALHOST, Ipv4Addr::UNSPECIFIED);
/// let arp_packet_bytes: Vec<u8> = (&arp_packet).into();
/// arp_socket.send(&arp_packet_bytes).unwrap();
/// ```
#[derive(Debug)]
#[repr(C, packed)]
pub struct ArpPacket {
    pub hardware_type: ArpHardwareType,
    pub protocol_type: ArpProtocolType,
    pub hw_addr_len: u8,
    pub proto_addr_len: u8,
    pub operation: ArpOperationCode,
    pub sender_mac: MacAddress,
    pub sender_ip: [u8; 4],
    pub target_mac: MacAddress,
    pub target_ip: [u8; 4],
}

/// Error type for converting bytes into an ArpPacket
#[derive(Error, Debug)]
pub enum ArpPacketConvertError {
    /// The input has an invalid length.
    #[error("Input has {0} bytes instead of 28")]
    InvalidLength(usize),

    /// A u16 field could not be converted into its corresponding enum.
    #[error("Invalid enum value: {0}")]
    InvalidEnum(#[from] ArpEnumConvertError),
}

impl Default for ArpPacket {
    fn default() -> Self {
        Self {
            hardware_type: ArpHardwareType::Reserved,
            protocol_type: ArpProtocolType::Reserved,
            hw_addr_len: 0,
            proto_addr_len: 0,
            operation: ArpOperationCode::Reserved,
            sender_mac: MacAddress::ZERO,
            sender_ip: Ipv4Addr::UNSPECIFIED.octets(),
            target_mac: MacAddress::ZERO,
            target_ip: Ipv4Addr::UNSPECIFIED.octets(),
        }
    }
}

impl From<&ArpPacket> for Vec<u8> {
    fn from(value: &ArpPacket) -> Self {
        let mut bytes = Vec::with_capacity(28);
        bytes.extend_from_slice(&(value.hardware_type as u16).to_be_bytes());
        bytes.extend_from_slice(&(value.protocol_type as u16).to_be_bytes());
        bytes.push(value.hw_addr_len);
        bytes.push(value.proto_addr_len);
        bytes.extend_from_slice(&(value.operation as u16).to_be_bytes());
        bytes.extend_from_slice(&value.sender_mac.into_array());
        bytes.extend_from_slice(&value.sender_ip);
        bytes.extend_from_slice(&value.target_mac.into_array());
        bytes.extend_from_slice(&value.target_ip);
        bytes
    }
}

impl TryFrom<&[u8]> for ArpPacket {
    type Error = ArpPacketConvertError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let bytes_amount = value.len();
        if bytes_amount != 28 {
            return Err(ArpPacketConvertError::InvalidLength(bytes_amount));
        }

        Ok(Self {
            hardware_type: ArpHardwareType::try_from(u16::from_be_bytes([value[0], value[1]]))?,
            protocol_type: ArpProtocolType::try_from(u16::from_be_bytes([value[2], value[3]]))?,
            hw_addr_len: value[4],
            proto_addr_len: value[5],
            operation: ArpOperationCode::try_from(u16::from_be_bytes([value[6], value[7]]))?,
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

impl ArpPacket {
    /// Construct an ArpPacket by only providing the sender's MacAddress, IP, and the target's IP.
    ///
    /// This is the most common use of the ARP protocol, finding the MacAddress of a device by
    /// knowing its IP address
    pub const fn request(sender_mac: MacAddress, sender_ip: Ipv4Addr, target_ip: Ipv4Addr) -> Self {
        Self {
            hardware_type: ArpHardwareType::Ethernet,
            protocol_type: ArpProtocolType::Ipv4,
            hw_addr_len: 6,
            proto_addr_len: 4,
            operation: ArpOperationCode::Request,
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
            hardware_type: ArpHardwareType::Ethernet,
            protocol_type: ArpProtocolType::Ipv4,
            hw_addr_len: 6,
            proto_addr_len: 4,
            operation: ArpOperationCode::Reply,
            sender_mac,
            sender_ip: sender_ip.octets(),
            target_mac,
            target_ip: target_ip.octets(),
        }
    }

    /// Checks if the ArpPacket is a request (op code 1)
    pub const fn is_request(&self) -> bool {
        matches!(self.operation, ArpOperationCode::Request)
    }

    /// Checks if the ArpPacket is a reply (op code 2)
    pub const fn is_reply(&self) -> bool {
        matches!(self.operation, ArpOperationCode::Reply)
    }
}
