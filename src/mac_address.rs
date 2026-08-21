use std::{
    fmt::{Display, LowerHex, UpperHex},
    str::FromStr,
};
use thiserror::Error;

/// The error type for Parsing Mac Addresses.
#[derive(Error, Debug)]
pub enum ParseMacAddressError {
    #[error("MAC Address has {0} octets instead of 6")]
    InvalidLength(usize),

    #[error("MAC Address octets are invalid")]
    InvalidOctet,
}

/// A struct to work with MAC Addresses.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MacAddress {
    octets: [u8; 6],
}

impl From<[u8; 6]> for MacAddress {
    fn from(value: [u8; 6]) -> Self {
        MacAddress::new(value)
    }
}

impl MacAddress {
    /// A Broadcast MAC Address.
    pub const BROADCAST: Self = Self::new([0xFF; 6]);

    /// A MAC Address with all octets being 0.
    pub const ZERO: Self = Self::new([0x00; 6]);

    /// Construct a new MAC Address given an array of six octets.
    pub const fn new(octets: [u8; 6]) -> Self {
        MacAddress { octets }
    }

    /// Get all the MAC Addresses of all interfaces on this device.
    ///
    // This may include MAC addresses belonging to virtual interfaces,
    // as a MAC address alone does not indicate whether its interface is physical or virtual.
    pub fn get_all_device_macs() -> Vec<Self> {
        use libc::{AF_PACKET, freeifaddrs, getifaddrs};
        use std::ptr;

        let mut result = Vec::new();

        unsafe {
            let mut ifaddrs = ptr::null_mut();

            if getifaddrs(&mut ifaddrs) != 0 {
                return result;
            }

            let mut current = ifaddrs;

            while !current.is_null() {
                let addr = (*current).ifa_addr;

                if !addr.is_null() && (*addr).sa_family as i32 == AF_PACKET {
                    let sll = addr as *const libc::sockaddr_ll;

                    if (*sll).sll_halen == 6 {
                        let mac = Self::new([
                            (*sll).sll_addr[0] as u8,
                            (*sll).sll_addr[1] as u8,
                            (*sll).sll_addr[2] as u8,
                            (*sll).sll_addr[3] as u8,
                            (*sll).sll_addr[4] as u8,
                            (*sll).sll_addr[5] as u8,
                        ]);

                        if !mac.is_zero() {
                            result.push(mac);
                        }
                    }
                }

                current = (*current).ifa_next;
            }

            freeifaddrs(ifaddrs);
        }

        result.sort();
        result.dedup();

        result
    }

    /// Filter through all the MAC Addresses of all interfaces on this device.
    ///
    /// The closure should return true if the MacAddress should be kept
    ///
    // This may include MAC addresses belonging to virtual interfaces,
    // as a MAC address alone does not indicate whether its interface is physical or virtual.
    pub fn get_all_device_macs_filter<F: FnMut(&MacAddress) -> bool>(mut filter_f: F) -> Vec<Self> {
        use libc::{AF_PACKET, freeifaddrs, getifaddrs};
        use std::ptr;

        let mut result = Vec::new();

        unsafe {
            let mut ifaddrs = ptr::null_mut();

            if getifaddrs(&mut ifaddrs) != 0 {
                return result;
            }

            let mut current = ifaddrs;

            while !current.is_null() {
                let addr = (*current).ifa_addr;

                if !addr.is_null() && (*addr).sa_family as i32 == AF_PACKET {
                    let sll = addr as *const libc::sockaddr_ll;

                    if (*sll).sll_halen == 6 {
                        let mac = Self::new([
                            (*sll).sll_addr[0] as u8,
                            (*sll).sll_addr[1] as u8,
                            (*sll).sll_addr[2] as u8,
                            (*sll).sll_addr[3] as u8,
                            (*sll).sll_addr[4] as u8,
                            (*sll).sll_addr[5] as u8,
                        ]);

                        if mac.is_zero() {
                            continue;
                        }

                        if !filter_f(&mac) {
                            continue;
                        }

                        result.push(mac);
                    }
                }

                current = (*current).ifa_next;
            }

            freeifaddrs(ifaddrs);
        }

        result.sort();
        result.dedup();

        result
    }

    /// Borrow the raw octet array.
    pub const fn octets(&self) -> &[u8; 6] {
        &self.octets
    }

    /// Consume the MacAddress into raw octet array.
    pub const fn into_array(self) -> [u8; 6] {
        self.octets
    }

    /// Check if the MAC Address is a broadcast MAC Address.
    pub fn is_broadcast(&self) -> bool {
        self == &Self::BROADCAST
    }

    /// Check if all octets are zero.
    pub fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }

    /// Check if the MAC Address is multicast.
    pub const fn is_multicast(&self) -> bool {
        (self.octets[0] & 0x01) != 0
    }

    /// Check if the MAC Address is unicast.
    pub const fn is_unicast(&self) -> bool {
        !self.is_multicast()
    }

    /// Check if the MAC Address is local.
    pub const fn is_local(&self) -> bool {
        (self.octets[0] & 0x02) != 0
    }

    /// Check if the MAC Address is universal.
    pub const fn is_universal(&self) -> bool {
        !self.is_local()
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d, e, s] = self.octets;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            a, b, c, d, e, s
        )
    }
}

impl LowerHex for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d, e, s] = self.octets;
        write!(f, "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}", a, b, c, d, e, s)
    }
}

impl UpperHex for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d, e, s] = self.octets;
        write!(f, "{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}", a, b, c, d, e, s)
    }
}

impl FromStr for MacAddress {
    type Err = ParseMacAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_octets = [0u8; 6];
        for (i, hex_oct) in s.split(':').enumerate() {
            if i >= 6 {
                return Err(ParseMacAddressError::InvalidLength(i + 1));
            }
            num_octets[i] =
                u8::from_str_radix(hex_oct, 16).map_err(|_| ParseMacAddressError::InvalidOctet)?;
        }
        if s.split(':').count() != 6 {
            return Err(ParseMacAddressError::InvalidLength(s.split(':').count()));
        }
        Ok(MacAddress { octets: num_octets })
    }
}
