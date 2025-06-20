/*!
	Symetric encryption module.
*/

// External stuff:
use serde::Deserialize;
use std::fmt::Display;

// Module declarations go here:
#[cfg(feature = "aes-gcm")]
mod qsh_aes_gcm;

// Re-export them here:
#[cfg(feature = "aes-gcm")]
pub use qsh_aes_gcm::{AesGcmEncryptor, AesGcmDecryptor};

use crate::kex::{KeyExchanger, self};

pub trait Encryptor {
	type Error: Display;


	fn new<T: KeyExchanger>(key_exchange: T) -> Self;

	fn encrypt(&mut self, data: &mut Vec<u8>, adata: &[u8]) -> Result<(), Self::Error>;

}
pub trait Decryptor {
	type Error: Display;


	fn new<T: KeyExchanger>(key_exchange: T) -> Self;

	fn decrypt(&mut self, data: &mut Vec<u8>, adata: &[u8]) -> Result<(), Self::Error>;

}


/// Types of encryption.
#[derive(Deserialize)]
pub enum Implementation {
	AesGcm,
} impl Implementation {
	pub fn generate<T: KeyExchanger>(&self, i: T, o: T) -> (impl Encryptor + use<T>, impl Decryptor + use<T>) {
		return match self {
			Self::AesGcm => (AesGcmEncryptor::new(o), AesGcmDecryptor::new(i)),
		};
	}
}