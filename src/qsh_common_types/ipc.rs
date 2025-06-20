/*!
	Stuff for IPC between `qsh` and `qshd`.
*/
use std::{
	path::PathBuf,
	net::Ipv6Addr,
};
use bincode::{
	Encode,
	Decode,
	config,
};

pub const IPC_BINCODE_CONFIG: config::Configuration = config::standard();

#[derive(Encode, Decode)]
pub enum ChannelDirection {
	I,
	O,
}

#[derive(Encode, Decode)]
pub enum ChannelType {
	Unbuffered,
	Buffered,
}

/// Sent from `qsh` to `qshd` to start a session.
#[derive(Encode, Decode)]
pub struct SessionRequest {

	// Who to connect to:
	remote_host: Ipv6Addr,

	// Port:
	port: u16,

	// What to run on the other end:
	execute: PathBuf,

} impl SessionRequest {
	pub fn new<T: Into<Ipv6Addr>, U: Into<PathBuf>>(remote_host: T, port:u16, execute: U) -> Self {
		return Self {
			remote_host: remote_host.into(),
			port: port,
			execute: execute.into(),
		};
	}
}

/// Sent from `qshd` to `qsh`, contains details about the new session.
#[derive(Encode, Decode)]
pub struct SessionAcknowledge {

	pub id: u16,

	// Path to the session control socket:
	pub socket_path: PathBuf,

	// Path to stdin:
	pub stdin_path: PathBuf,

	// Path to stdout:
	pub stdout_path: PathBuf,

	// Path to stderr:
	pub stderr_path: PathBuf,

} impl SessionAcknowledge {
	pub fn new<T: Into<PathBuf>>(id: u16, socket_path: T, stdin_path: T, stdout_path: T, stderr_path: T) -> Self {
		return Self {
			id: id,
			socket_path: socket_path.into(),
			stdin_path: stdin_path.into(),
			stdout_path: stdout_path.into(),
			stderr_path: stderr_path.into(),
		};
	}
}

/// Sent from `qsh` to `qshd` to create a new channel.
#[derive(Encode, Decode)]
pub struct ChannelRequest {

	// Which direction?
	direction: ChannelDirection,

	// Buffered or interactive?
	channel_type: ChannelType,

} impl ChannelRequest {
	pub fn new(direction: ChannelDirection, channel_type: ChannelType) -> Self {
		return Self {
			direction: direction,
			channel_type: channel_type,
		};
	}
}

/// Sent from `qshd` to `qsh`, contains details about the new channel.
#[derive(Encode, Decode)]
pub struct ChannelAcknowledge {

	id: u16,

	// Path to the channel control socket:
	socket_path: PathBuf,

} impl ChannelAcknowledge {
	pub fn new<T: Into<PathBuf>>(id: u16, socket_path: T) -> Self {
		return Self {
			id: id,
			socket_path: socket_path.into(),
		};
	}
}