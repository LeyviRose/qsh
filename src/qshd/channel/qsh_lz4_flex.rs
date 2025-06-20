/*!
	Implements the default compression method, lz4_flex (LZ4).
*/

// External dependancies go here:
use lz4_flex::frame;
use tokio::{
	sync::{
		mpsc::{
			Sender,
			Receiver,
		},
	},
};
use std::path::PathBuf;

// Internal dependancies go here:
use super::Channel;


pub struct Lz4FlexChannel;

impl Channel for Lz4FlexChannel {
	
	async fn i_bound(receiver: Receiver<Vec<u8>>, socket_path: &PathBuf) {
		todo!()
	}

	async fn o_bound(socket_path: &PathBuf, sender: Sender<Vec<u8>>) {
		todo!()
	}

}

// Tests go at the bottom:
#[test]
fn test_lz4_flex() {
	todo!()
}
