# rawnet

Simple to use crate to easily work with Raw Sockets and Packets.

## Overview

`rawnet` provides safe abstractions over raw and packet sockets for working with networking at OSI layers 2 and 3.
It includes a `Socket` trait for a unified interface across socket types, plus a `MacAddress` type for MAC address manipulation.

## Features

- **`Socket` trait** -- common interface for `send`, `receive`, and `connect` across socket types
- **`ArpSocket`** -- AF_PACKET socket for sending and receiving ARP packets (layer 2/3)
- **`LocalSocket`** -- AF_LOCAL socket for Unix domain IPC (layer 4)
- **`MacAddress`** -- 6-byte MAC address type with parsing, formatting, and classification (broadcast, multicast, unicast, local/universal)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rawnet = "0.1.0"
```

or using `cargo` in your project's directory:

```bash
cargo add rawnet
```

## Example

```rust
use rawnet::MacAddress;
use std::str::FromStr;

let mac = MacAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap();
assert!(!mac.is_broadcast());
assert!(mac.is_unicast());
```

## Platform

Linux only. Uses `AF_PACKET` (Linux-specific) for packet sockets.

## License

MIT
