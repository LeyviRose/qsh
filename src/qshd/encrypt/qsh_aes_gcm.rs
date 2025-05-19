/*!
	Implements the default symetric encryption scheme (AES-256-GCM).
	Encryption method 0.
*/

// External dependancies go here:
use aes_gcm::{self, aead::AeadMutInPlace, Aes256Gcm, Error, KeyInit, Nonce};

// Internal dependancies go here:
use super::Crypto;

pub struct AesGcmCrypto {

	// Cipher struct. Does all the exciting stuff:
	cipher: Aes256Gcm,

}


impl Crypto for AesGcmCrypto {
	type Error = Error;

	fn new<T: crate::kex::KeyExchanger>(key_exchange: T) -> Self {
		return Self {
			cipher: Aes256Gcm::new(key_exchange.shared_secret().into()),
		};
	}

	fn encrypt(&mut self, data: &mut Vec<u8>, adata: &[u8], nonce: &[u8]) -> Result<(), Self::Error> {
		// We'll just call this function on the data:
		self.cipher.encrypt_in_place(Nonce::from_slice(nonce), adata, data)?;
		
		return Ok(());
	}

	fn decrypt(&mut self, data: &mut Vec<u8>, adata: &[u8], nonce: &[u8]) -> Result<(), Self::Error> {
		// Same thing, in reverse:
		self.cipher.decrypt_in_place(Nonce::from_slice(nonce), adata, data)?;
		
		return Ok(());
	}

}

#[test]
fn test_aes_gcm_crypto() {
	use crate::kex::{KeyExchanger, KyberlibKeyExchanger};

	// Run a key exchange:

	let mut alice_kex: KyberlibKeyExchanger = KyberlibKeyExchanger::new().expect("Failed to create Alice in AES-GCM test");
	let mut bob_kex: KyberlibKeyExchanger = KyberlibKeyExchanger::new().expect("Failed to create Bob in AES-GCM test");

	alice_kex.set_remote_pubkey(bob_kex.get_local_pubkey()).expect("Failed to set Alice's remote pubkey in AES-GCM test");
	bob_kex.set_remote_pubkey(alice_kex.get_local_pubkey()).expect("Failed to set Bob's remote pubkey in AES-GCM test");

	let alice_init: [u8; 2272] = alice_kex.client_init().expect("Client init failed for Alice in AES-GCM test");
	alice_kex.client_confirm(bob_kex.server_init(alice_init).expect("Server init failed for Bob in AES-GCM test")).expect("Client confirm failed for Alice in AES-GCM test");


	// Let's go test it!

	let mut alice: AesGcmCrypto = AesGcmCrypto::new(alice_kex);
	let mut bob: AesGcmCrypto = AesGcmCrypto::new(bob_kex);

	let mut alice_msg: Vec<u8> = b"Hello, Bob!".into();
	let mut bob_response: Vec<u8> = b"Hello, Alice!".into();

	let first_nonce: [u8; 12] = [0; 12];
	let second_nonce: [u8; 12] = [1; 12];

	alice.encrypt(&mut alice_msg, b"", &first_nonce).expect("Failed to encrypt Alice's message in AES-GCM test");
	bob.decrypt(&mut alice_msg, b"", &first_nonce).expect("Failed to decrypt Alice's message in AES-GCM test");

	bob.encrypt(&mut bob_response, b"", &second_nonce).expect("Failed to encrypt Bob's message in AES-GCM test");
	alice.decrypt(&mut bob_response, b"", &second_nonce).expect("Failed to decrypt Bob's response in AES-GCM test");
}