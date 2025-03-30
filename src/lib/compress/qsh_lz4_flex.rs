/*!
	Implements the default compression scheme, lz4_flex (LZ4).
	Compression option 0 in the protocol.
*/

// External dependancies go here:
use lz4_flex::block;

// Internal dependancies go here:
use super::Compressor;


pub struct Lz4FlexCompressor;

impl Compressor for Lz4FlexCompressor {
	type Error = block::DecompressError;
	
	fn new() -> Self {
		Self{}
	}

	fn compress(&self, payload: &[u8]) -> Vec<u8> {
		block::compress_prepend_size(payload)
	}

	fn decompress(&self, payload: &[u8]) -> Result<Vec<u8>, Self::Error> {
		Ok(block::decompress_size_prepended(payload)?)
	}
	
}

// Tests go at the bottom:
#[test]
fn test_lz4_flex() {
	let testing_data: &[u8; 14] = b"Hello, world!\n";
	let compressor: Lz4FlexCompressor = Lz4FlexCompressor::new();
	assert_eq!(compressor.decompress(&compressor.compress(testing_data)).expect("Something is very wrong with lz4_flex!"), testing_data, "lz4_flex failed to decompress data it compressed!");
}
