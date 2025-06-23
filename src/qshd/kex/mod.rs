/*!
	Key-exchange module.
*/

use serde::Deserialize;
use std::{array::TryFromSliceError, fmt::Display, mem::size_of};

// Module declarations go here:
#[cfg(feature = "kyberlib")]
pub mod qsh_kyberlib;

// Re-export them here:
#[cfg(feature = "kyberlib")]
pub use qsh_kyberlib::KyberlibKeyExchanger;

pub trait KeyExchanger {
	type Error: Display;
	type ClientInit;
	type ServerInit;
	type PublicKey: TryInto<Vec<u8>>;

	const CI_LEN: usize = size_of::<Self::ClientInit>();
	const SI_LEN: usize = size_of::<Self::ServerInit>();
	const PK_LEN: usize = size_of::<Self::PublicKey>();


	fn new() -> Result<Self, Self::Error> where Self: Sized;

	/// Exports the local pubkey, so that it can be sent to the remote host.
	/// Run this when you want to start a key exchange; both parties having
	/// the other's public key is a necissary step in key exchanging.
	fn get_local_pubkey(&self) -> Vec<u8>;

	/// Returns the length of a client init.
	fn get_client_init_length(&self) -> usize {
		return Self::CI_LEN;
	}

	/// Returns the length of a server init.
	fn get_server_init_length(&self) -> usize {
		return Self::SI_LEN;
	}

	/// Returns the length of a public key.
	fn get_public_key_length(&self) -> usize {
		return Self::PK_LEN;
	}

	/// Set a remote host public key.
	/// This is run using the output of the above function, on the other
	/// side of the connection.
	fn set_remote_pubkey(&mut self, pubkey: &[u8]) -> Result<(), TryFromSliceError>;

	/// Performs a client-side init.
	/// (note that this can also be called on the server side,
	/// client here really means "initiator")
	/// Requires that the server's public key is already here, and saved in
	/// the structure using `set_remote_pubkey()`.
	fn client_init(&mut self) -> Result<Vec<u8>, Self::Error>;

	/// Generate the server response. Requires the client's public key, and their request for key exchange.
	fn server_init(&mut self, client_init: &[u8]) -> Result<Vec<u8>, Self::Error>;

	/// Confirm it! Requires the server's response to the request for key exchange.
	fn client_confirm(&mut self, server_init: &[u8]) -> Result<(), Self::Error>;

	/// The whole point: a shared secret.
	fn shared_secret(&self) -> &[u8];

}


/// Types of key exchange.
#[derive(Deserialize)]
pub enum Implementation {
	Kyberlib,
} impl Implementation {
	/// Generates a key exchanger dynamically from the configuration `struct`.
	pub fn generate(&self) -> impl KeyExchanger + use<> {
		return match self {
			Self::Kyberlib => KyberlibKeyExchanger::new().expect("failed to generate new keypair"),
		};
	}
}