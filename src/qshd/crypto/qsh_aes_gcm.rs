/*!
	Implements the default symetric encryption scheme (AES-256-GCM).
	Encryption method 0.
*/

// External dependancies go here:
use aes_gcm::{self, aead::AeadMutInPlace, Aes256Gcm, Error, KeyInit, Nonce};
use arbitrary_int::u96;

// Internal dependancies go here:
use super::{Encryptor, Decryptor};

pub struct AesGcmEncryptor {

	cipher: Aes256Gcm,
	nonce: u96,

}
pub struct AesGcmDecryptor {

	cipher: Aes256Gcm,
	nonce: u96,

}


impl Encryptor for AesGcmEncryptor {
	type Error = Error;

	fn new<T: crate::kex::KeyExchanger>(key_exchange: T) -> Self {
		return Self {
			cipher: Aes256Gcm::new(key_exchange.shared_secret().into()),
			nonce: u96::from_u64(0),
		};
	}

	fn encrypt(&mut self, data: &mut Vec<u8>, adata: &[u8]) -> Result<(), Self::Error> {
		// We'll just call this function on the data (note the little-endian specific method used here):
		self.cipher.encrypt_in_place(Nonce::from_slice(&self.nonce.to_le_bytes()), adata, data)?;
		
		// Increment outbound nonce:
		self.nonce += u96::from_u64(1);

		return Ok(());
	}

}
impl Decryptor for AesGcmDecryptor {
	type Error = Error;

	fn new<T: crate::kex::KeyExchanger>(key_exchange: T) -> Self {
		return Self {
			cipher: Aes256Gcm::new(key_exchange.shared_secret().into()),
			nonce: u96::from_u64(0),
		};
	}

	fn decrypt(&mut self, data: &mut Vec<u8>, adata: &[u8]) -> Result<(), Self::Error> {
		// Same thing, in reverse (note the little-endian specific method used here):
		self.cipher.decrypt_in_place(Nonce::from_slice(&self.nonce.to_le_bytes()), adata, data)?;
		
		// Increment inbound nonce:
		self.nonce += u96::from_u64(1);

		return Ok(());
	}

}

#[test]
fn test_aes_gcm_crypto() {
	use crate::kex::{KeyExchanger, KyberlibKeyExchanger};

	// Run a double key exchange:

	let mut alice_kex_o: KyberlibKeyExchanger = KyberlibKeyExchanger::new().expect("Failed to create Alice in AES-GCM test");
	let mut alice_kex_i: KyberlibKeyExchanger = KyberlibKeyExchanger::new().expect("Failed to create Alice in AES-GCM test");

	let mut bob_kex_o: KyberlibKeyExchanger = KyberlibKeyExchanger::new().expect("Failed to create Bob in AES-GCM test");
	let mut bob_kex_i: KyberlibKeyExchanger = KyberlibKeyExchanger::new().expect("Failed to create Bob in AES-GCM test");

	alice_kex_o.set_remote_pubkey(&bob_kex_i.get_local_pubkey()).expect("Failed to set Alice's remote pubkey in AES-GCM test");
	alice_kex_i.set_remote_pubkey(&bob_kex_o.get_local_pubkey()).expect("Failed to set Alice's remote pubkey in AES-GCM test");
	bob_kex_o.set_remote_pubkey(&alice_kex_i.get_local_pubkey()).expect("Failed to set Bob's remote pubkey in AES-GCM test");
	bob_kex_i.set_remote_pubkey(&alice_kex_o.get_local_pubkey()).expect("Failed to set Bob's remote pubkey in AES-GCM test");

	let alice_init: Vec<u8> = alice_kex_o.client_init().expect("Client init failed for Alice in AES-GCM test");
	let bob_init: Vec<u8> = bob_kex_o.client_init().expect("Client init failed for Bob in AES-GCM test");
	alice_kex_o.client_confirm(&bob_kex_i.server_init(&alice_init).expect("Server init failed for Bob in AES-GCM test")).expect("Client confirm failed for Alice in AES-GCM test");
	bob_kex_o.client_confirm(&alice_kex_i.server_init(&bob_init).expect("Server init failed for Bob in AES-GCM test")).expect("Client confirm failed for Bob in AES-GCM test");


	// Let's go test it!

	let mut alice_en: AesGcmEncryptor = AesGcmEncryptor::new(alice_kex_o);
	let mut alice_de: AesGcmDecryptor = AesGcmDecryptor::new(alice_kex_i);
	let mut bob_en: AesGcmEncryptor = AesGcmEncryptor::new(bob_kex_o);
	let mut bob_de: AesGcmDecryptor = AesGcmDecryptor::new(bob_kex_i);

	let mut alice_msg: Vec<u8> = b"Hello, Bob!".into();
	let mut bob_response: Vec<u8> = b"Hello, Alice!".into();

	alice_en.encrypt(&mut alice_msg, b"").expect("Failed to encrypt Alice's message in AES-GCM test");
	bob_de.decrypt(&mut alice_msg, b"").expect("Failed to decrypt Alice's message in AES-GCM test");

	bob_en.encrypt(&mut bob_response, b"").expect("Failed to encrypt Bob's message in AES-GCM test");
	alice_de.decrypt(&mut bob_response, b"").expect("Failed to decrypt Bob's response in AES-GCM test");
}