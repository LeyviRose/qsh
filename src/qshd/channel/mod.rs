/*!
	Channels module.
*/
use tokio::{
	sync::{
		mpsc::{
			Sender,
			Receiver,
		},
	},
};
use std::path::PathBuf;
use serde::Deserialize;

// Module declarations go here:
#[cfg(feature = "lz4_flex")]
mod qsh_lz4_flex;

// Re-export them here:
#[cfg(feature = "lz4_flex")]
pub use qsh_lz4_flex::Lz4FlexChannel;

pub trait Channel {

	/// Forwards bytes from `receiver` to `socket_path`.
	async fn i_bound(receiver: Receiver<Vec<u8>>, socket_path: &PathBuf);

	/// Forwards bytes from `socket_path` to `sender`.
	async fn o_bound(socket_path: &PathBuf, sender: Sender<Vec<u8>>);

}

#[derive(Deserialize)]
struct ChannelConfig {

	/// Which implementation to use.
	#[serde(default = "default_implementation")]
	implementation: Implementation,

	/// How many miliseconds to wait between checking the sockets for new data.
	#[serde(default = "default_poll_interval")]
	poll_interval: u16,

}

#[derive(Deserialize)]
pub enum Implementation {
	Lz4Flex,
} /* impl Implementation {
	pub fn generate(&self, config: ChannelConfig) -> impl Channel {
		return match self {
			Implementation::Lz4Flex => Lz4FlexChannel::new(ChannelConfig),
		};
	}
} */

fn default_poll_interval() -> u16 {
	return 250;
}
fn default_implementation() -> Implementation {
	return Implementation::Lz4Flex;
}