use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct MacAddress {
    octets: [u8; 6],
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

    pub fn is_broadcast(&self) -> bool {
        self.octets() == Self::BROADCAST.octets()
    }

    pub fn is_zero(&self) -> bool {
        self.octets() == Self::ZERO.octets()
    }

    pub fn is_multicast(&self) -> bool {
        (self.octets[0] & 0x01) != 0
    }

    pub fn is_unicast(&self) -> bool {
        !self.is_multicast()
    }

    pub fn is_local(&self) -> bool {
        (self.octets[0] & 0x02) != 0
    }

    pub fn is_universal(&self) -> bool {
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
