/*!
	Implements the default key exchange method, kyberlib (CRYSTALS-Kyber).
	Key exchange method 0.
*/

use std::array::TryFromSliceError;

// External dependancies go here:
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;
use kyberlib::{keypair, Ake, AkeSendInit, AkeSendResponse, Keypair, KyberLibError, PublicKey};

// Internal dependancies go here:
use super::KeyExchanger;


pub struct KyberlibKeyExchanger {

	// Generator for random numbers:
	random: ChaCha20Rng,

	// Stores the state for the key exchange:
	state: Ake,

	// Stores the keypair for this exchange:
	keypair: Keypair,

	// Stores the public key of the remote host:
	remote_pubkey: Option<PublicKey>,

}

impl KeyExchanger for KyberlibKeyExchanger {
	type Error = KyberLibError;
	type ClientInit = AkeSendInit;
	type ServerInit = AkeSendResponse;
	type PublicKey = PublicKey;

	fn new() -> Result<Self, Self::Error> {
		let mut random: ChaCha20Rng = ChaCha20Rng::from_entropy();
		let state: Ake = Ake::new();
		let keypair: Keypair = keypair(&mut random)?;

		return Ok(Self {
			random: random,
			state: state,
			keypair: keypair,
			remote_pubkey: None,
		});
	}
	
	fn get_local_pubkey(&self) -> Self::PublicKey {
		return self.keypair.public;
	}

	fn set_remote_pubkey(&mut self, pubkey: Self::PublicKey) -> Result<(), TryFromSliceError> {
		self.remote_pubkey = Some(PublicKey::from(pubkey.try_into()?));
		return Ok(());
	}

	fn client_init(&mut self) -> Result<Self::ClientInit, Self::Error> {
		// Check if there's a public key stored here yet:
		if let Some(pubkey) = self.remote_pubkey {
			// If there is, run `client_init`, propagate any errors, and return the data as an owned Vec:
			return Ok(self.state.client_init(&pubkey, &mut self.random)?);
		} else {
			// Or return this error (`MissingKey` would be more appropriate, but that doesn't exist😁):
			return Err(KyberLibError::InvalidKey);
		}
	}

	fn server_init(&mut self, client_init: Self::ClientInit) -> Result<Self::ServerInit, Self::Error> {
		// Check if there's a public key:
		if let Some(pubkey) = self.remote_pubkey {
			// If yes, `server_init`:
			return Ok(self.state.server_receive(client_init, &pubkey, &self.keypair.secret, &mut self.random)?);
		} else {
			// If not, error:
			return Err(KyberLibError::InvalidKey);
		}
	}

	fn client_confirm(&mut self, server_init: Self::ServerInit) -> Result<(), Self::Error> {
		// Final step: propagate errors:
		self.state.client_confirm(server_init, &self.keypair.secret)?;
		// Or return `Ok`:
		return Ok(());
	}

	fn shared_secret(&self) -> &[u8] {
		return &self.state.shared_secret;
	}

}

#[test]
fn test_kyberlib_key_exchanger() {
	// Test constructor:
	let mut alice: KyberlibKeyExchanger = KyberlibKeyExchanger::new().expect("Failed to create `alice`!");
	let mut bob: KyberlibKeyExchanger = KyberlibKeyExchanger::new().expect("Failed to create `bob`!");
	// Exchange public keys:
	bob.set_remote_pubkey(alice.get_local_pubkey()).expect("Failed to set `bob`'s pubkey!");
	alice.set_remote_pubkey(bob.get_local_pubkey()).expect("Failed to set `alice`'s pubkey!");
	// Alice is the client. Test `client_init`:
	let client_init: <KyberlibKeyExchanger as KeyExchanger>::ClientInit = alice.client_init().expect("Failed to initialize client `alice`!");
	// Bob is the server. Test `server_init`:
	let server_init: <KyberlibKeyExchanger as KeyExchanger>::ServerInit = bob.server_init(client_init).expect("Failed to initialize server `bob`!");
	// Check it:
	alice.client_confirm(server_init).expect("Failed to confirm client `alice`!");
}