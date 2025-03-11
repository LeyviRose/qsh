use lz4_flex::block;

use super::{Compressor, QshDecompressError};

pub struct Lz4FlexCompressor;

impl Lz4FlexCompressor {
	pub fn new() -> Self {
		Self{}
	}
}

impl Compressor for Lz4FlexCompressor {
	
	fn compress(&self, payload: &[u8]) -> Vec<u8> {
		block::compress_prepend_size(payload)
	}

	fn decompress(&self, payload: &[u8]) -> Result<Vec<u8>, QshDecompressError> {
		Ok(block::decompress_size_prepended(payload)?)
	}
	
}

impl From<block::DecompressError> for QshDecompressError {
	fn from(error: block::DecompressError) -> Self {
		QshDecompressError::Lz4FlexError(error)
	}
}
