/*!
	Implements the default key exchange method, kyberlib (CRYSTALS-Kyber).
	Key exchange method 0.
*/

// External dependancies go here:
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use kyberlib::{keypair, Ake, Keypair, KyberLibError};

// Internal dependancies go here:
use super::KeyExchanger;


pub struct KyberlibKeyExchanger {

	// Generator for random numbers:
	random: ChaCha20Rng,

	// Stores the state for the key exchange:
	state: Ake,

	// Stores the keypair for this exchange:
	keypair: Keypair,

}

impl KeyExchanger for KyberlibKeyExchanger {
	type Error = KyberLibError;

	fn new() -> Result<Self, Self::Error> {
		let mut random: ChaCha20Rng = ChaCha20Rng::from_os_rng();
		let state: Ake = Ake::new();
		let keypair: Keypair = keypair(&mut random)?;

		Ok(Self{
			random: random,
			state: state,
			keypair: keypair,
		})
	}

	fn client_init(&self, server_pubkey: &[u8]) -> Result<Vec<u8>, Self::Error> {
		todo!()
	}

	fn server_init(&self, client_pubkey: &[u8], client_init: &[u8]) -> Result<Vec<u8>, Self::Error> {
		todo!()
	}

	fn client_ack(&self, server_init: &[u8]) -> Result<Vec<u8>, Self::Error> {
		todo!()
	}

	fn shared_secret(&self) -> &[u8] {
		todo!()
	}

}