pub trait Packet: TryFrom<Vec<u8>> {
    fn ether_type(&self) -> u16;
}
