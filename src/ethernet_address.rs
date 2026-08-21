use crate::MacAddress;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

/// The error type for Parsing Interfaces
#[derive(Error, Debug)]
pub enum ParseInterfaceError {
    #[error("Interface name exceeds 15 characters")]
    InvalidLength,

    #[error("Interface name contains a space")]
    ContainsSpace,

    #[error("Interface name contains a slash (/)")]
    ContainsSlash,

    #[error("Interface name contains a colon (:)")]
    ContainsColon,
}

/// An Ethernet Interface, contains a [`String`] which is the name of the interface.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Interface(String);

impl Interface {
    /// Return a reference to the interface name
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Interface {
    type Err = ParseInterfaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 15 {
            return Err(ParseInterfaceError::InvalidLength);
        } else if s.contains(' ') {
            return Err(ParseInterfaceError::ContainsSpace);
        } else if s.contains('/') {
            return Err(ParseInterfaceError::ContainsSlash);
        } else if s.contains(':') {
            return Err(ParseInterfaceError::ContainsColon);
        }

        Ok(Self(s.to_string()))
    }
}

/// An Ethernet Address (not to be confused with [`MacAddress`]).
///
/// Contrarty to common terminology, this struct
/// contains both a MAC Address and an Interface.
/// this struct's purpose is to be used within [`EthernetSocket`]
/// to send raw ethernet frames.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EthernetAddress {
    pub mac_address: MacAddress,
    pub interface: Interface,
}

impl EthernetAddress {
    /// Construct a new Ethernet Address given a [`MacAddress`] and an [`Interface`].
    pub fn new(mac_address: MacAddress, interface: impl Into<Interface>) -> Self {
        let interface = interface.into();

        Self {
            mac_address,
            interface,
        }
    }
}

impl Display for EthernetAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ethernet Interface Name: {}", self.interface)?;
        write!(f, "Ethernet MAC Address: {}", self.mac_address)?;
        Ok(())
    }
}
