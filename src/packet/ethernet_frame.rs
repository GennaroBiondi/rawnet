use crate::{MacAddress, packet::Packet};

pub struct EthernetFrame<P: Packet> {
    pub source_mac: MacAddress,
    pub destination_mac: MacAddress,
    pub payload_packet: P,
}

impl<P: Packet> EthernetFrame<P> {
    pub fn new(source_mac: MacAddress, destination_mac: MacAddress, payload_packet: P) -> Self {
        Self {
            source_mac,
            destination_mac,
            payload_packet,
        }
    }
}
