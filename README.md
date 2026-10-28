# zigzag-rs

Zigzag encoding/decoding for primitive integers.

## Usage

```rust
use zigzag_rs::ZigZag;

let encoded: u32 = i32::encode(-3);
assert_eq!(encoded, 5);
assert_eq!(i32::decode(encoded), -3);
```

`ZigZag` is implemented for `i8`, `i16`, `i32`, `i64`, `i128` and `isize`,
each paired with the unsigned type of the same width.

## Features

- `#![no_std]`

---

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in zigzag-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>