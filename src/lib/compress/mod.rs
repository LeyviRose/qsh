// Package `use`s go here:
use thiserror::Error;

// Modules go here:
pub(self) mod qsh_lz4_flex;

// Re-export them here:
pub use self::qsh_lz4_flex::Lz4FlexCompressor;

trait Compressor {
	fn compress(&self, payload: &[u8]) -> Vec<u8>;
	fn decompress(&self, payload: &[u8]) -> Result<Vec<u8>, QshDecompressError>;
}

#[derive(Error, Debug)]
pub enum QshDecompressError {
	#[error("Failed to decompress with lz4_flex: {0}")]
	Lz4FlexError(lz4_flex::block::DecompressError)
}

#[cfg(test)]
mod tests;
