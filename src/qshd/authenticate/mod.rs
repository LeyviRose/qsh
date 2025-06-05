/*!
	Authentication module.
*/

use std::net::Ipv6Addr;

#[cfg(feature = "fips204")]
mod qsh_fips204;

#[cfg(feature = "fips204")]
pub use qsh_fips204::Fips204Authenticator;

trait Authenticator {
	type Error;
	type Signature;


	async fn new() -> Self;

	/// Sign some data with the local private key.
	fn sign(&mut self, data: &[u8]) -> Self::Signature;

	/// Verify some data signed with a remote public key.
	/// Returns true if everything checks out.
	fn verify(&self, data: &[u8], host: Ipv6Addr, signature: &Self::Signature) -> bool;

}