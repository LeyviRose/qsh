/*!
	Stuff here is meant to manage the connection to the service.
*/

use std::{
	collections::HashMap,
	net::Ipv6Addr,
	path::PathBuf,
};
use bincode::{
	self, config, Encode
};
use tokio::{
	fs::{
		self,
		File,
	},
	io::{AsyncReadExt, AsyncWriteExt, Result},
	net::{
		self,
		UnixStream,
	},
};

use qsh_common_types::ipc::*;

/// A channel; one data stream.
struct Channel {

	// ID; each channel has a session-unique number:
	id: u16,

	// Which direction?
	direction: ChannelDirection,

	// Interactive (unbuffered) or buffered?
	channel_type: ChannelType,
	
	// This is where I/O happens:
	socket: UnixStream,

}


/// A session; connection to remote host, possibly proxied.
struct Session {

	// ID: (later)
	id: u16,

	// Remote host:
	host: Ipv6Addr,

	// The socket used to control the session:
	socket: UnixStream,

	// Channel list:
	channels: HashMap<u16, Channel>,

	// Executable to run:
	executable: PathBuf,

}


pub struct Daemon {

	// Socket for control:
	socket: UnixStream,

	// Session:
	session: Session,
}


impl Daemon {
	pub async fn new(path: PathBuf, host: Ipv6Addr, port: u16, executable: PathBuf) -> Result<Self> {
		//! `path`: path to the socket in XDG_RUNTIME_DIR that manages the daemon.
		//! `host`: IPv6 address of the remote host.
		
		// Connect to the socket:
		let mut socket: UnixStream = UnixStream::connect(path).await?;

		// Send a session request:
		socket.write_all(bincode::encode_to_vec(SessionRequest::new(host, port, &executable), IPC_BINCODE_CONFIG).unwrap().as_slice()).await?;

		// Make new session struct:
		socket.readable().await?;	// Wait for the socket to become readable.
		let mut buffer: Vec<u8> = vec![0; socket.read_u64_le().await? as usize];
		socket.read_exact(buffer.as_mut_slice()).await?;
		let response: SessionAcknowledge = bincode::decode_from_slice(&buffer, IPC_BINCODE_CONFIG).unwrap().0;
		let mut channels: HashMap<u16, Channel> = HashMap::new();
		
		// Add stdin:
		channels.insert(0, Channel {
			id: 0,
			direction: ChannelDirection::O,
			channel_type: ChannelType::Unbuffered,
			socket: UnixStream::connect(response.stdin_path).await?,
		});

		// Add stdout:
		channels.insert(1, Channel {
			id: 1,
			direction: ChannelDirection::I,
			channel_type: ChannelType::Unbuffered,
			socket: UnixStream::connect(response.stdout_path).await?,
		});

		// Add stderr:
		channels.insert(2, Channel {
			id: 2,
			direction: ChannelDirection::I,
			channel_type: ChannelType::Unbuffered,
			socket: UnixStream::connect(response.stderr_path).await?,
		});


		return Ok(Self {
			socket: socket,
			session: Session {
				id: response.id,
				host: host,
				socket: UnixStream::connect(response.socket_path).await?,
				channels: channels,
				executable: executable,
			},
		});
	}
}

/*
	TODO:
	- Make new channels. (Delayed; omit for now.)
*/