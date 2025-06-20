/*!
	For handling the server's configuration.
*/
use serde::Deserialize;
use toml;
use std::{
	net::Ipv6Addr,
	path::PathBuf,
	env,
};
use tokio::{
	fs,
};

use super::super::types::config::{
	channel::ChannelConfiguration,
	session::{KeyTypes, SessionConfiguration},
	connection::ConnectionConfiguration,
};


/// Server's configuration.
#[derive(Deserialize)]
pub struct ServerConfiguration {

	/// What executable to run (default: /bin/sh).
	#[serde(default = "default_exec")]
	exec: String,

	/// Connection-related settings.
	connection_settings: ConnectionConfiguration,

	/// Session-related settings.
	session_settings: SessionConfiguration,

	/// Channel-related settings.
	channel_settings: ChannelConfiguration,

	/// List of clients allowed to connect and their public keys. If none are present, the server will not run.
	clients: Option<Vec<Client>>,

}

#[derive(Deserialize)]
struct Client {
	
	/// Who (later may support domains).
	addr: Ipv6Addr,

	/// What kind of key.
	key_type: KeyTypes,

	/// The name of the key.
	key_name: String,

}


fn default_exec() -> String {
	return String::from("/bin/sh");
}


impl ServerConfiguration {
	pub async fn load() -> Option<Self> {
		let config_path: PathBuf = PathBuf::from(env::var("HOME").unwrap()).join(".qsh/server.toml");
		let config_data: String = fs::read_to_string(&config_path).await.expect("failed to open/read configuration file");
		let configuration: ServerConfiguration = toml::from_str(&config_data).expect("failed to parse configuration file");

		if let Some(_) = configuration.clients {
			return Some(configuration);
		} else {
			return None;
		}
	}
}