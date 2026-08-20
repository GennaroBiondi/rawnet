use std::str::FromStr;

use crate::mac_address::ParseMacAddressError;

use super::*;

#[test]
fn mac_address_from_str_valid() {
    MacAddress::from_str("AA:BB:CC:DD:EE:FF").expect("MacAddress string parsing failed!");
}

#[test]
fn mac_address_from_str_invalid_length() {
    let mac_addr: Result<MacAddress, ParseMacAddressError> =
        MacAddress::from_str("AA:BB:CC:DD:EE:FF:GG");

    assert!(
        matches!(mac_addr, Err(ParseMacAddressError::InvalidLength(7))),
        "MacAddress of invalid length was considered valid!"
    );
}

#[test]
fn mac_address_from_str_invalid_octet() {
    let mac_addr: Result<MacAddress, ParseMacAddressError> =
        MacAddress::from_str("00:bb:@a:53:1:x");

    assert!(
        matches!(mac_addr, Err(ParseMacAddressError::InvalidOctet)),
        "MacAddress with invalid octets was considered valid!"
    );
}

#[test]
fn mac_address_display() {
    let mac_addr = MacAddress::from_str("AA:BB:CC:DD:EE:FF").unwrap();
    assert!(
        mac_addr.to_string() == "aa:bb:cc:dd:ee:ff",
        "MacAddress converted to string is wrong!"
    );
    let upper_mac = format!("{:X}", mac_addr);
    assert!(
        upper_mac == "AABBCCDDEEFF",
        "MacAddress converted to upper string is wrong!"
    );
}
