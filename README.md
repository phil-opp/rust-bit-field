# bit_field

A simple crate which provides the `BitField` trait, which provides methods for operating on individual bits and ranges
of bits on Rust's integral types.

## Documentation
Documentation is available on [docs.rs](https://docs.rs/bit_field/0.7.0/bit_field/index.html)

## Usage
```TOML
[dependencies]
bit_field = "0.9.0"
```

## Example
```rust
extern crate bit_field;
use bit_field::BitField;

let mut x: u8 = 0;
let msb = x.bit_length() - 1;

x.set_bit(msb, true);
assert_eq!(x, 0b1000_0000);

x.set_bits(0..4, 0b1001);
assert_eq!(x, 0b1000_1001);

```

## License
This crate is dual-licensed under MIT or the Apache License (Version 2.0). See LICENSE-APACHE and LICENSE-MIT for details.
