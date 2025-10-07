#![no_std]

//! # zigzag-rs
//!
//! Zigzag encoding/decoding for primitive integers.

use num_traits::{AsPrimitive, PrimInt, Signed, Unsigned, One};

pub trait ZigZag: Signed + PrimInt
where Self: AsPrimitive<Self::UInt> {
	type UInt: Unsigned + PrimInt + AsPrimitive<Self>;

	/// Encode a signed integer into an unsigned integer using zigzag encoding.
	///
	/// Zigzag transforms signed integers into unsigned ones, assigning compact
	/// codes to values with low absolute magnitude, regardless of sign:
	///
	/// `0 → 0, -1 → 1, 1 → 2, -2 → 3, ...`
	#[inline]
	fn encode(value: Self) -> Self::UInt {
		let s = (value << 1) ^ (value >> ((size_of::<Self>() * 8) - 1));
		s.as_()
	}

	/// Decode an unsigned zigzag-encoded integer back into its signed value.
	///
	/// This reverses the transformation performed by [encode](ZigZag::encode).
	#[inline]
	fn decode(value: Self::UInt) -> Self {
		let shr = value >> 1;
		let lsb_mask = (value & One::one()).as_();
		let neg = (-lsb_mask).as_();
		let or = shr ^ neg;
		or.as_()
	}
}


macro_rules! impl_zigzag {
    ($signed:ty, $unsigned:ty) => {
        impl ZigZag for $signed {
            type UInt = $unsigned;
        }
    };
}

impl_zigzag!(i8, u8);
impl_zigzag!(i16, u16);
impl_zigzag!(i32, u32);
impl_zigzag!(i64, u64);
impl_zigzag!(i128, u128);
impl_zigzag!(isize, usize);


#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn all_i16_values() {
		for i in i16::MIN..=i16::MAX {
			let zig = ZigZag::encode(i);
			let zag = ZigZag::decode(zig);
			assert_eq!(i, zag);
		}
	}
}
