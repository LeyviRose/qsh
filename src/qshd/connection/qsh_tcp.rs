/*!
	A way to transport data over TCP.
*/

// External stuff:
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter, Error},
	net::{
		tcp::{ OwnedReadHalf, OwnedWriteHalf, }, TcpListener, TcpSocket
	},
	sync::mpsc::{
			self, Receiver, Sender
	},
	task,
};
use std::{net::Ipv6Addr, vec};

// Internal stuff:
use super::{Connection, ConnectionConfiguration};
use crate::{
	crypto::{Encryptor, Decryptor},
	kex::KeyExchanger,
};


pub struct TcpConnection {
	listener: Option<TcpListener>,
	config: ConnectionConfiguration,
} impl TcpConnection {

	/// This performs the key exchange, returning the resulting encryptor/decryptor pair, or an error.
	async fn exchange_keys(&self, tx: &mut BufWriter<OwnedWriteHalf>, rx: &mut BufReader<OwnedReadHalf>) -> Result<(impl Encryptor + use<>, impl Decryptor + use<>), Error> {
		
		// First we need to make two key exchange objects:
		let mut i_kex = self.config.kex.generate();
		let mut o_kex = self.config.kex.generate();

		// We'll send `i_kex`'s public key first, then `o_kex`'s:
		tx.write(&i_kex.get_local_pubkey()).await?;
		tx.write(&o_kex.get_local_pubkey()).await?;
		tx.flush().await?;

		// Then, we read out the remote public keys:
		let mut i_pubkey_buf: Vec<u8> = vec![0_u8; i_kex.get_public_key_length()];
		let mut o_pubkey_buf: Vec<u8> = vec![0_u8; o_kex.get_public_key_length()];
		rx.read_exact(&mut o_pubkey_buf).await?;	// The other's output should be my input,
		rx.read_exact(&mut i_pubkey_buf).await?;	// and vis-versa.

		// And set the remote public key on the key exchangers:
		i_kex.set_remote_pubkey(&i_pubkey_buf).map_err(|e| { Error::other(e) })?;
		o_kex.set_remote_pubkey(&o_pubkey_buf).map_err(|e| { Error::other(e) })?;

		// Now we need to actually initiate the key exchange, starting with the client init step:
		tx.write(&o_kex.client_init().map_err(|e| { Error::other(e.to_string()) })?).await?;	// Send a client init.
		tx.flush().await?;
		let mut i_remote_client_init_buf: Vec<u8> = vec![0_u8; i_kex.get_client_init_length()];	// For holding the client's client init.
		rx.read_exact(&mut i_remote_client_init_buf).await?;	// Receive it.

		// Do the server init step:
		tx.write(&i_kex.server_init(&i_remote_client_init_buf).map_err(|e| { Error::other(e.to_string()) })?).await?;	// Send a server init.
		tx.flush().await?;
		let mut o_remote_server_init_buf: Vec<u8> = vec![0_u8; o_kex.get_server_init_length()];	// For holding the client's server init.
		rx.read_exact(&mut o_remote_server_init_buf).await?;

		// Do the client confirm step:
		o_kex.client_confirm(&o_remote_server_init_buf).map_err(|e| { Error::other(e.to_string()) })?;	// Done with key exchange!

		// Make the keys/crypto thingies:
		return Ok(self.config.crypto.generate(i_kex, o_kex));
	}

	/**
		Used to spin up a send task.
		`tx`: socket to send on.
		`ch`: channel to read out of.
		`en`: `Encryptor` to use.
	*/
	async fn send_task<T: Encryptor>(tx: &mut BufWriter<OwnedWriteHalf>, ch: &mut Receiver<Vec<u8>>, en: &mut T) {
		if let Err(e) = loop {
			// Read byte vectors out of the channel, until the other end is dropped:
			if let Some(mut byte_vec) = ch.recv().await {
				if let Err(e) = en.encrypt(&mut byte_vec, b"").map_err(|e| { Error::other(e.to_string()) }) {
					// Attempt to encrypt the data ☝🏻
					break Err(e);
				} else if let Err(e) = tx.write_u64_le(byte_vec.len().try_into().unwrap()).await {
					// Send the length ☝🏻
					break Err(e);
				} else if let Err(e) = tx.write(&byte_vec).await {
					// Send the data ☝🏻
					break Err(e);
				} else if let Err(e) = tx.flush().await {
					// Flush the buffer ☝🏻
					break Err(e);
				}
			} else {
				// Run if the channel is closed:
				break tx.shutdown().await;
			}
		} {
			// This gets run if an error occures in the loop:
			eprintln!("an error occured on a sender task connected to {}: {}", tx.get_ref().peer_addr().unwrap(), e);
		} else {
			// This gets run if all is dandy:
			eprintln!("Closed sender task connected to {}", tx.get_ref().peer_addr().unwrap());
		}
		return;
	}

	/**
		Used to spin up a receive task.
		`rx`: socket to receive on.
		`ch`: channel to send to.
		`de`: `Decryptor` to use.
	*/
	async fn recv_task<T: Decryptor>(rx: &mut BufReader<OwnedReadHalf>, ch: &mut Sender<Vec<u8>>, de: &mut T) {
		if let Err(e) = loop {
			// First, read the length of the next message:
			if let Ok(message_length) = rx.read_u64_le().await {
				// Make a buffer to store the message:
				let mut buf: Vec<u8> = vec![0_u8; message_length.try_into().unwrap()];
				// Read it:
				if let Err(e) = rx.read_exact(&mut buf).await {
					// If receiving fails, exit the loop:
					break Err(e);
				} else if let Err(e) = de.decrypt(&mut buf, b"").map_err(|e| { Error::other(e.to_string()) }) {
					// Try to decrypt the message, and if it fails:
					break Err(e);
				} else if let Err(_) = ch.send(buf).await {
					// Try to send the decrypted message down the channel, and if that fails, then the `Receiver` must have been dropped, so the connection should be closed:
					break Ok(());
				}
			} else {
				// Run if there's an error reading the length from the socket:
				break Err(Error::last_os_error());
			}
		} {
			// Run if an error occures in the loop above:
			eprintln!("an error occured on a receiver task connected to {}: {}", rx.get_ref().peer_addr().unwrap(), e);
		} else {
			// If all is well:
			eprintln!("closed receiver task connected to {}", rx.get_ref().peer_addr().unwrap());
		}
		return;
	}

}

impl Connection for TcpConnection {
	type Error = tokio::io::Error;


	fn new(config: ConnectionConfiguration) -> Self {
		return Self {
			listener: None,
			config: config,
		};
	}

	async fn listen(&mut self) -> Result<(), Self::Error> {
		// Bind a socket and set the field with it:
		let sock: TcpSocket = TcpSocket::new_v6()?;	// IPv6.
		sock.set_reuseport(true)?;	// So that all connections can use the same port.
		sock.bind((self.config.addr, self.config.port).into())?;	// Actually bind it.
		self.listener = Some(sock.listen(1)?);
		eprintln!("Server listening on [{}]:{}.", self.config.addr, self.config.port);
		return Ok(());
	}

	async fn accept(&mut self) -> Result<(Sender<Vec<u8>>, Receiver<Vec<u8>>), Self::Error> {
		
		// Check if we're supposed to be listening:
		if let Some(listener) = &self.listener {

			// If we are, accept one connection:
			let (stream, address) = listener.accept().await?;
			eprintln!("Server: new connection from {}.", &address);

			// And split the stream into a sender and a receiver:
			let (rx_u, tx_u) = stream.into_split();
			let mut tx: BufWriter<OwnedWriteHalf> = BufWriter::new(tx_u);
			let mut rx: BufReader<OwnedReadHalf> = BufReader::new(rx_u);

			// Make the keys/crypto thingies:
			let (mut encryptor, mut decryptor) = self.exchange_keys(&mut tx, &mut rx).await?;

			// Make the channels:
			let (send_sender, mut send_receiver) = mpsc::channel::<Vec<u8>>(Self::CHANNEL_BUFFER_SIZE);
			let (mut recv_sender, recv_receiver) = mpsc::channel::<Vec<u8>>(Self::CHANNEL_BUFFER_SIZE);

			// Spawn the tasks, and add their abort handles to the internal vector:
			task::spawn(async move { Self::send_task(&mut tx, &mut send_receiver, &mut encryptor).await });	// Send task.
			task::spawn(async move { Self::recv_task(&mut rx, &mut recv_sender, &mut decryptor).await });	// Receive task.

			// Return the channels:
			return Ok((send_sender, recv_receiver));
		} else {
			// If this `struct` _shouldn't_ be listening:
			panic!("this `TcpConnection` is not listening!");
		}

	}

	async fn connect(&mut self, addr: Ipv6Addr, port: u16) -> Result<(Sender<Vec<u8>>, Receiver<Vec<u8>>), Self::Error> {
		
		// First, make sure that this isn't supposed to be a server:
		if let None = self.listener {
			// All good? Continue by setting up a socket:
			let sock: TcpSocket = TcpSocket::new_v6()?;	// IPv6.
			sock.set_reuseport(true)?;	// So that multiple connections can use the same port.
			sock.bind((self.config.addr, self.config.port).into())?;	// Use the configured listen address and port to connect to the remote server.

			// Now connect, and split the stream:
			let (rx_u, tx_u) = sock.connect((addr, port).into()).await?.into_split();

			// And buffer them:
			let mut tx: BufWriter<OwnedWriteHalf> = BufWriter::new(tx_u);
			let mut rx: BufReader<OwnedReadHalf> = BufReader::new(rx_u);

			// Make the encryptor and decryptor:
			let (mut encryptor, mut decryptor) = self.exchange_keys(&mut tx, &mut rx).await?;

			// Make the channels:
			let (send_sender, mut send_receiver) = mpsc::channel::<Vec<u8>>(Self::CHANNEL_BUFFER_SIZE);
			let (mut recv_sender, recv_receiver) = mpsc::channel::<Vec<u8>>(Self::CHANNEL_BUFFER_SIZE);

			// Spawn the tasks, adding their abort handles to the vector:
			task::spawn(async move { Self::send_task(&mut tx, &mut send_receiver, &mut encryptor).await });	// Send task.
			task::spawn(async move { Self::recv_task(&mut rx, &mut recv_sender, &mut decryptor).await });	// Receive task.

			return Ok((send_sender, recv_receiver));
		} else {
			// If this `struct` shouldn't be connecting:
			panic!("this `TcpConnection` should not be connecting!");
		}

	}

}


#[tokio::test]
async fn test_tcp_connection() {

	let message: &[u8] = b"The missile knows where it is at all times; it knows this because it knows where it isn't.";
	eprintln!("Client's message: {}", str::from_utf8(message).unwrap());

	// Need a configuration first:
	let client_conf: ConnectionConfiguration = ConnectionConfiguration {
		addr: Ipv6Addr::LOCALHOST,
		port: 54320,
		connection: super::Implementation::Tcp,
		crypto: crate::crypto::Implementation::AesGcm,
		kex: crate::kex::Implementation::Kyberlib,
	};
	let server_conf: ConnectionConfiguration = ConnectionConfiguration {
		addr: Ipv6Addr::LOCALHOST,
		port: 54321,
		connection: super::Implementation::Tcp,
		crypto: crate::crypto::Implementation::AesGcm,
		kex: crate::kex::Implementation::Kyberlib,
	};

	// Make a server:
	let mut server: TcpConnection = TcpConnection::new(server_conf);
	
	// Make a client:
	let mut client: TcpConnection = TcpConnection::new(client_conf);

	// Runs the server:
	task::spawn(async move {
		server.listen().await.unwrap();
		let (tx, mut rx) = server.accept().await.unwrap();
		
		// Simple echo server:
		while let Some(data) = rx.recv().await {
			eprintln!("Server received: {}", str::from_utf8(&data).unwrap());
			tx.send(data).await.unwrap();
		}
		return;
	});
	tokio::time::sleep(std::time::Duration::from_secs(1)).await;	// So that the server has time to start.

	// Runs a client:
	let (ctx, mut crx) = client.connect(Ipv6Addr::LOCALHOST, 54321).await.unwrap();
	ctx.send(message.to_vec()).await.unwrap();
	let response: Vec<u8> = crx.recv().await.unwrap();
	eprintln!("Client heard: {}", str::from_utf8(&response).unwrap());
	assert_eq!(response, message.to_vec());
}