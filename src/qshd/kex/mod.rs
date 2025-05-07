// Module declarations go here:
#[cfg(feature = "kyberlib")]
pub mod qsh_kyberlib;

use std::array::TryFromSliceError;

// Re-export them here:
#[cfg(feature = "kyberlib")]
pub use qsh_kyberlib::KyberlibKeyExchanger;

pub trait KeyExchanger {
	type Error;
	type ClientInit;
	type ServerInit;

	fn new() -> Result<Self, Self::Error> where Self: Sized;

	/// Exports the local pubkey, so that it can be sent to the remote host.
	/// Run this when you want to start a key exchange; both parties having
	/// the other's public key is a necissary step in key exchanging.
	fn get_local_pubkey(&self) -> &[u8];
	
	/// Set a remote host public key.
	/// This is run using the output of the above function, on the other
	/// side of the connection.
	fn set_remote_pubkey(&mut self, pubkey: &[u8]) -> Result<(), TryFromSliceError>;

	/// Performs a client-side init.
	/// (note that this can also be called on the server side,
	/// client here really means "initiator")
	/// Requires that the server's public key is already here, and saved in
	/// the structure using `set_remote_pubkey()`.
	fn client_init(&mut self) -> Result<Self::ClientInit, Self::Error>;

	/// Generate the server response. Requires the client's public key, and their request for key exchange.
	fn server_init(&mut self, client_init: Self::ClientInit) -> Result<Self::ServerInit, Self::Error>;

	/// Confirm it! Requires the server's response to the request for key exchange.
	fn client_confirm(&mut self, server_init: Self::ServerInit) -> Result<(), Self::Error>;

	/// The whole point: a shared secret.
	fn shared_secret(&self) -> &[u8];

}
