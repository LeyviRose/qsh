use super::{Compressor, Lz4FlexCompressor};

#[test]
fn test_compressors() {
	test_lz4_flex();
}

fn test_lz4_flex() {
	let testing_data: &[u8; 14] = b"Hello, world!\n";
	let compressor: Lz4FlexCompressor = Lz4FlexCompressor::new();
	assert_eq!(compressor.decompress(&compressor.compress(testing_data)).expect("Something is very wrong with lz4_flex!"), testing_data, "lz4_flex failed to decompress data it compressed!");
}