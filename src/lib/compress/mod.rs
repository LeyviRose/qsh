// Module declarations go here:
#[cfg(feature = "lz4_flex")]
mod qsh_lz4_flex;

// Re-export them here:
#[cfg(feature = "lz4_flex")]
pub use qsh_lz4_flex::Lz4FlexCompressor;

pub trait Compressor {
	type Error;

	fn new() -> Self;

	/// Compress `payload`.
	fn compress(&self, payload: &[u8]) -> Vec<u8>;

	/// Decompress `payload`.
	fn decompress(&self, payload: &[u8]) -> Result<Vec<u8>, Self::Error>;
}
