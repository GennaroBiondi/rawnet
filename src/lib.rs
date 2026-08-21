//! This crate's purpose is to make working with Raw Sockets and Packet Sockets simple.
mod ethernet_address;
mod mac_address;

/// Module for working with protocol packets
pub mod packet;
/// Module for working with Sockets in OSI layer 2 and 3.
pub mod socket;

pub use mac_address::MacAddress;

#[cfg(test)]
mod tests;
