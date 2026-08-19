use std::{
    fmt::{Display, LowerHex, UpperHex},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseMacAddressError {
    #[error("Input doesn't have exactly 6 octets")]
    InvalidLength,

    #[error("Input octets are invalid")]
    InvalidOctet,
}

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
    pub const BROADCAST: Self = Self::new([0xFF; 6]);
    pub const ZERO: Self = Self::new([0x00; 6]);

    pub const fn new(octets: [u8; 6]) -> Self {
        MacAddress { octets }
    }

    /// Borrow the raw octet array
    pub const fn octets(&self) -> &[u8; 6] {
        &self.octets
    }

    /// Consume the MacAddress into raw octet array
    pub const fn into_array(self) -> [u8; 6] {
        self.octets
    }

    pub const fn is_broadcast(&self) -> bool {
        let [a, b, c, d, e, f] = self.octets;
        a == 0xFF && b == 0xFF && c == 0xFF && d == 0xFF && e == 0xFF && f == 0xFF
    }

    pub const fn is_zero(&self) -> bool {
        let [a, b, c, d, e, f] = self.octets;
        a == 0 && b == 0 && c == 0 && d == 0 && e == 0 && f == 0
    }

    pub const fn is_multicast(&self) -> bool {
        (self.octets[0] & 0x01) != 0
    }

    pub const fn is_unicast(&self) -> bool {
        !self.is_multicast()
    }

    pub const fn is_local(&self) -> bool {
        (self.octets[0] & 0x02) != 0
    }

    pub const fn is_universal(&self) -> bool {
        !self.is_local()
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d, e, g] = self.octets;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            a, b, c, d, e, g
        )
    }
}

impl LowerHex for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d, e, g] = self.octets;
        write!(f, "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}", a, b, c, d, e, g)
    }
}

impl UpperHex for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d, e, g] = self.octets;
        write!(f, "{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}", a, b, c, d, e, g)
    }
}

impl FromStr for MacAddress {
    type Err = ParseMacAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let octets: Vec<&str> = s.split(':').collect();
        let octets_amount = octets.len();

        if octets_amount != 6 {
            return Err(ParseMacAddressError::InvalidLength);
        }

        let mut num_octets: [u8; 6] = [0; 6];

        for (i, hex_oct) in octets.iter().enumerate() {
            num_octets[i] =
                u8::from_str_radix(hex_oct, 16).map_err(|_| ParseMacAddressError::InvalidOctet)?
        }

        Ok(MacAddress { octets: num_octets })
    }
}
