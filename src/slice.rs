use crate::zigzag::ZigZag;
use num_traits::AsPrimitive;

pub fn encode_in_place<T: ZigZag>(buf: &mut [T]) {
	for v in buf {
		*v = T::encode(*v).as_();
	}
}

pub fn decode_in_place<T: ZigZag>(buf: &mut [T::UInt]) {
	for v in buf {
		*v = T::decode(*v).as_();
	}
}

#[derive(thiserror::Error, Debug)]
pub enum IntoBufferError {
	#[error("Destination buffer is too small")]
	DstTooSmall,
}

fn encode_into<T: ZigZag>(src: &[T], dst: &mut [T::UInt]) -> Result<(), IntoBufferError> {
	if dst.len() < src.len() {
		return Err(IntoBufferError::DstTooSmall);
	}

	for (s, d) in src.iter().zip(dst.iter_mut()) {
		*d = T::encode(*s);
	}

	Ok(())
}

fn decode_into<T: ZigZag>(src: &[T::UInt], dst: &mut [T]) -> Result<(), IntoBufferError> {
	if dst.len() < src.len() {
		return Err(IntoBufferError::DstTooSmall);
	}

	for (s, d) in src.iter().zip(dst.iter_mut()) {
		*d = T::decode(*s);
	}

	Ok(())
}
