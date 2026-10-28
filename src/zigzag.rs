use num_traits::{AsPrimitive, One, PrimInt, Signed, Unsigned};

mod sealed {
	pub trait Sealed {}
}

/// Zigzag encoding for signed primitive integers.
///
/// This trait is sealed and implemented for `i8`, `i16`, `i32`, `i64`,
/// `i128` and `isize`, each paired with the unsigned type of the same width.
///
/// # Examples
///
/// ```
/// use zigzag_rs::ZigZag;
///
/// assert_eq!(i32::encode(-1), 1u32);
/// assert_eq!(i32::decode(1u32), -1);
/// ```
pub trait ZigZag: sealed::Sealed + Signed + PrimInt + AsPrimitive<Self::UInt> {
	/// The unsigned integer type of the same width as `Self`.
	type UInt: Unsigned + PrimInt + AsPrimitive<Self>;

	/// Encode a signed integer into an unsigned integer using zigzag encoding.
	///
	/// # Examples
	///
	/// ```
	/// use zigzag_rs::ZigZag;
	///
	/// assert_eq!(i8::encode(0), 0);
	/// assert_eq!(i8::encode(-1), 1);
	/// assert_eq!(i8::encode(1), 2);
	/// assert_eq!(i8::encode(-2), 3);
	/// assert_eq!(i8::encode(i8::MIN), u8::MAX);
	/// ```
	#[inline]
	fn encode(value: Self) -> Self::UInt {
		let bits = size_of::<Self>() * 8;
		let sign = value >> (bits - 1);
		((value << 1) ^ sign).as_()
	}

	/// Decode an unsigned zigzag-encoded integer back into its signed value.
	///
	/// This reverses the transformation performed by [`encode`](ZigZag::encode).
	///
	/// # Examples
	///
	/// ```
	/// use zigzag_rs::ZigZag;
	///
	/// assert_eq!(i16::decode(0), 0);
	/// assert_eq!(i16::decode(3), -2);
	/// assert_eq!(i16::decode(4), 2);
	/// assert_eq!(i16::decode(u16::MAX), i16::MIN);
	/// ```
	#[inline]
	fn decode(value: Self::UInt) -> Self {
		let lsb: Self = (value & Self::UInt::one()).as_();
		let sign: Self::UInt = (-lsb).as_();
		((value >> 1) ^ sign).as_()
	}
}

macro_rules! impl_zigzag {
	($signed:ty, $unsigned:ty) => {
		const _: () = assert!(<$signed>::BITS == <$unsigned>::BITS);

		impl sealed::Sealed for $signed {}

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
	fn i16_values() {
		for i in i16::MIN..=i16::MAX {
			let zig = ZigZag::encode(i);
			let zag = ZigZag::decode(zig);
			assert_eq!(i, zag);
		}
	}

	#[test]
	fn i8_values() {
		for i in i8::MIN..=i8::MAX {
			let zig = ZigZag::encode(i);
			let zag = ZigZag::decode(zig);
			assert_eq!(i, zag);
		}
	}

	macro_rules! sample_values {
		($name:ident, $signed:ty, $unsigned:ty) => {
			#[test]
			fn $name() {
				let cases: &[($signed, $unsigned)] = &[
					(0, 0),
					(-1, 1),
					(1, 2),
					(-2, 3),
					(2, 4),
					(<$signed>::MAX, <$unsigned>::MAX - 1),
					(<$signed>::MIN, <$unsigned>::MAX),
				];

				for &(signed, encoded) in cases {
					assert_eq!(encoded, ZigZag::encode(signed));
					assert_eq!(signed, ZigZag::decode(encoded));
				}
			}
		};
	}

	sample_values!(i8_samples, i8, u8);
	sample_values!(i16_samples, i16, u16);
	sample_values!(i32_samples, i32, u32);
	sample_values!(i64_samples, i64, u64);
	sample_values!(i128_samples, i128, u128);
	sample_values!(isize_samples, isize, usize);
}
