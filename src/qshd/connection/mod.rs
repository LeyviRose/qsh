/*!
	Abstraction layer on top of the transport layer (TCP, UDP, etc).
	Unbuffered.
*/
// External stuff:
use tokio::{
	sync::{
		mpsc::{
			Sender,
			Receiver,
		},
	},
};
use std::net::Ipv6Addr;
use serde::Deserialize;

// Stuff from other modules:
use super::{
	crypto,
	kex,
};

// Module declarations go here:
#[cfg(feature = "tcp")]
mod qsh_tcp;

#[cfg(feature = "tcp")]
pub use qsh_tcp::TcpConnection;


pub trait Connection: Sized {
	type Error;

	/// Length of the channel's buffer.
	const CHANNEL_BUFFER_SIZE: usize = 256;


	/// Make a new instance.
	fn new(config: ConnectionConfiguration) -> Self;

	/// Bind to an address and port with configuration `config`, as a server.
	async fn listen(&mut self) -> Result<(), Self::Error>;

	/// Accept an incoming connection (server only). Returns (remote host, (tx, rx)).
	async fn accept(&mut self) -> Result<(Sender<Vec<u8>>, Receiver<Vec<u8>>), Self::Error>;

	/// Connect to a server, as a client. Returns (tx, rx) on success.
	async fn connect(&mut self, addr: Ipv6Addr, port: u16) -> Result<(Sender<Vec<u8>>, Receiver<Vec<u8>>), Self::Error>;

}


/// Different types of connections available.
#[derive(Deserialize)]
pub enum Implementation {
	Tcp,
} impl Implementation {
	pub fn generate(&self, config: ConnectionConfiguration) -> impl Connection {
		return match self {
			Self::Tcp => TcpConnection::new(config),
		};
	}
}

/// Settings for the connection layer.
#[derive(Deserialize)]
pub struct ConnectionConfiguration {

	/// What address to listen on.
	#[serde(default = "default_addr")]
	pub addr: Ipv6Addr,

	/// What port to listen on.
	#[serde(default = "default_port")]
	pub port: u16,

	/// Allowed types of connection.
	#[serde(default = "default_allowed_connection")]
	connection: Implementation,

	/// Allowed encryption types.
	#[serde(default = "default_allowed_crypto")]
	crypto: crypto::Implementation,

	/// Allowed key exchange types.
	#[serde(default = "default_allowed_kex")]
	kex: kex::Implementation,

}


fn default_allowed_connection() -> Implementation {
	return Implementation::Tcp;
}
fn default_allowed_crypto() -> crypto::Implementation {
	return crypto::Implementation::AesGcm;
}
fn default_allowed_kex() -> kex::Implementation {
	return kex::Implementation::Kyberlib;
}
fn default_addr() -> Ipv6Addr {
	return Ipv6Addr::LOCALHOST;
}
fn default_port() -> u16 {
	return 54321;
}