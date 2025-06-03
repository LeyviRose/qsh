/*!
	Implements the default symetric encryption scheme (AES-256-GCM).
	Encryption method 0.
*/

// External dependancies go here:
use aes_gcm::{self, aead::AeadMutInPlace, Aes256Gcm, Error, KeyInit, Nonce};
use arbitrary_int::u96;

// Internal dependancies go here:
use super::Crypto;

pub struct AesGcmCrypto {

	// Inbound:
	i_cipher: Aes256Gcm,
	i_nonce: u96,

	// Outbound:
	o_cipher: Aes256Gcm,
	o_nonce: u96,

}


impl Crypto for AesGcmCrypto {
	type Error = Error;

	fn new<T: crate::kex::KeyExchanger>(key_exchange_i: T, key_exchange_o: T) -> Self {
		return Self {
			i_cipher: Aes256Gcm::new(key_exchange_i.shared_secret().into()),
			i_nonce: u96::from_u64(0),
			o_cipher: Aes256Gcm::new(key_exchange_o.shared_secret().into()),
			o_nonce: u96::from_u64(0),
		};
	}

	fn encrypt(&mut self, data: &mut Vec<u8>, adata: &[u8]) -> Result<(), Self::Error> {
		// We'll just call this function on the data (note the little-endian specific method used here):
		self.o_cipher.encrypt_in_place(Nonce::from_slice(&self.o_nonce.to_le_bytes()), adata, data)?;
		
		// Increment outbound nonce:
		self.o_nonce += u96::from_u64(1);

		return Ok(());
	}

	fn decrypt(&mut self, data: &mut Vec<u8>, adata: &[u8]) -> Result<(), Self::Error> {
		// Same thing, in reverse (note the little-endian specific method used here):
		self.i_cipher.decrypt_in_place(Nonce::from_slice(&self.i_nonce.to_le_bytes()), adata, data)?;
		
		// Increment inbound nonce:
		self.i_nonce += u96::from_u64(1);

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

	alice_kex_o.set_remote_pubkey(bob_kex_i.get_local_pubkey()).expect("Failed to set Alice's remote pubkey in AES-GCM test");
	alice_kex_i.set_remote_pubkey(bob_kex_o.get_local_pubkey()).expect("Failed to set Alice's remote pubkey in AES-GCM test");
	bob_kex_o.set_remote_pubkey(alice_kex_i.get_local_pubkey()).expect("Failed to set Bob's remote pubkey in AES-GCM test");
	bob_kex_i.set_remote_pubkey(alice_kex_o.get_local_pubkey()).expect("Failed to set Bob's remote pubkey in AES-GCM test");

	let alice_init: [u8; 2272] = alice_kex_o.client_init().expect("Client init failed for Alice in AES-GCM test");
	let bob_init: [u8; 2272] = bob_kex_o.client_init().expect("Client init failed for Bob in AES-GCM test");
	alice_kex_o.client_confirm(bob_kex_i.server_init(alice_init).expect("Server init failed for Bob in AES-GCM test")).expect("Client confirm failed for Alice in AES-GCM test");
	bob_kex_o.client_confirm(alice_kex_i.server_init(bob_init).expect("Server init failed for Bob in AES-GCM test")).expect("Client confirm failed for Bob in AES-GCM test");


	// Let's go test it!

	let mut alice: AesGcmCrypto = AesGcmCrypto::new(alice_kex_i, alice_kex_o);
	let mut bob: AesGcmCrypto = AesGcmCrypto::new(bob_kex_i, bob_kex_o);

	let mut alice_msg: Vec<u8> = b"Hello, Bob!".into();
	let mut bob_response: Vec<u8> = b"Hello, Alice!".into();

	alice.encrypt(&mut alice_msg, b"").expect("Failed to encrypt Alice's message in AES-GCM test");
	bob.decrypt(&mut alice_msg, b"").expect("Failed to decrypt Alice's message in AES-GCM test");

	bob.encrypt(&mut bob_response, b"").expect("Failed to encrypt Bob's message in AES-GCM test");
	alice.decrypt(&mut bob_response, b"").expect("Failed to decrypt Bob's response in AES-GCM test");
}