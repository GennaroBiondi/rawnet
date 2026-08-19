use std::{
    fmt::{Display, LowerHex, UpperHex},
    str::FromStr,
};
use thiserror::Error;

/// The error type for Parsing Mac Addresses.
#[derive(Error, Debug)]
pub enum ParseMacAddressError {
    #[error("Input has {0} octets instead of 6")]
    InvalidLength(usize),

    #[error("Input octets are invalid")]
    InvalidOctet,
}

/// a struct to work with MAC Addresses.
///
/// [RFC 9724]: https://datatracker.ietf.org/doc/rfc9724
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MacAddress {
    octets: [u8; 6],
}

impl From<[u8; 6]> for MacAddress {
    fn from(value: [u8; 6]) -> Self {
        MacAddress::new(value)
    }
}

impl MacAddress {
    /// a Broadcast MAC Address.
    pub const BROADCAST: Self = Self::new([0xFF; 6]);

    /// a MAC Address with all octets being 0.
    pub const ZERO: Self = Self::new([0x00; 6]);

    /// Construct a new MAC Address given an array of six octets.
    pub const fn new(octets: [u8; 6]) -> Self {
        MacAddress { octets }
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

    /// Check if the MAC Address is local
    pub const fn is_local(&self) -> bool {
        (self.octets[0] & 0x02) != 0
    }

    /// Check if the MAC Address is not local
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
        let octets: Vec<&str> = s.split(':').collect();
        let octets_amount = octets.len();

        if octets_amount != 6 {
            return Err(ParseMacAddressError::InvalidLength(octets_amount));
        }

        let mut num_octets: [u8; 6] = [0; 6];

        for (i, hex_oct) in octets.iter().enumerate() {
            num_octets[i] =
                u8::from_str_radix(hex_oct, 16).map_err(|_| ParseMacAddressError::InvalidOctet)?
        }

        Ok(MacAddress { octets: num_octets })
    }
}
