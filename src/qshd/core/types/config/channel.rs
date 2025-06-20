use serde::Deserialize;


#[derive(Deserialize)]
pub enum ChannelTypes {
	Lz4Flex,
}

/// Settings for the channel layer.
#[derive(Deserialize)]
pub struct ChannelConfiguration {

	/// Number of miliseconds between checking for new data on the channel.
	#[serde(default = "default_poll_interval")]
	poll_interval: usize,

	/// Allowed channel types.
	#[serde(default = "default_allowed_channel")]
	channel: ChannelTypes,

}


fn default_allowed_channel() -> ChannelTypes {
	return ChannelTypes::Lz4Flex;
}
fn default_poll_interval() -> usize {
	return 100;
}