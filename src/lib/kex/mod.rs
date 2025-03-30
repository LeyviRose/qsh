// Module declarations go here:
#[cfg(feature = "kyberlib")]
pub mod qsh_kyberlib;

// Re-export them here:
#[cfg(feature = "kyberlib")]
pub use qsh_kyberlib::KyberlibKeyExchanger;

pub trait KeyExchanger {
	type Error;

	fn new() -> Result<Self, Self::Error>;

	/// Performs a client-side init. Requires the server's public key.
	fn client_init(&self, server_pubkey: &[u8]) -> Result<Vec<u8>, Self::Error>;

	/// Generate the server response. Requires the client's public key, and their request for key exchange.
	fn server_init(&self, client_pubkey: &[u8], client_init: &[u8]) -> Result<Vec<u8>, Self::Error>;

	/// Acknowledge it! Requires the server's response to the request for key exchange.
	fn client_ack(&self, server_init: &[u8]) -> Result<Vec<u8>, Self::Error>;

	/// The whole point: a shared secret.
	fn shared_secret(&self) -> &[u8];

}
