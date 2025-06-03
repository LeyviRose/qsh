// Module declarations go here:
#[cfg(feature = "aes-gcm")]
mod qsh_aes_gcm;

// Re-export them here:
#[cfg(feature = "aes-gcm")]
pub use qsh_aes_gcm::AesGcmCrypto;

use crate::kex::KeyExchanger;

pub trait Crypto {
	type Error;


	fn new<T: KeyExchanger>(key_exchange_i: T, key_exchange_o: T) -> Self;

	fn encrypt(&mut self, data: &mut Vec<u8>, adata: &[u8]) -> Result<(), Self::Error>;

	fn decrypt(&mut self, data: &mut Vec<u8>, adata: &[u8]) -> Result<(), Self::Error>;

}