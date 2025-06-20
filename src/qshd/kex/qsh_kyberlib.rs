/*!
	Implements the default key exchange method, kyberlib (CRYSTALS-Kyber).
	Key exchange method 0.
*/

// External dependancies go here:
use std::{
	array::TryFromSliceError,
};
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
	
	fn get_local_pubkey(&self) -> Vec<u8> {
		return self.keypair.public.clone().into();
	}

	fn set_remote_pubkey(&mut self, pubkey: &[u8]) -> Result<(), TryFromSliceError>{
		self.remote_pubkey = Some(pubkey.try_into()?);
		return Ok(());
	}

	fn client_init(&mut self) -> Result<Vec<u8>, Self::Error> {
		// Check if there's a public key stored here yet:
		if let Some(pubkey) = self.remote_pubkey {
			// If there is, run `client_init`, propagate any errors, and return the data as an owned Vec:
			return Ok(self.state.client_init(&pubkey, &mut self.random)?.into());
		} else {
			// Or return this error (`MissingKey` would be more appropriate, but that doesn't exist😁):
			return Err(KyberLibError::InvalidKey);
		}
	}

	fn server_init(&mut self, client_init: &[u8]) -> Result<Vec<u8>, Self::Error> {
		// Check if there's a public key:
		if let Some(pubkey) = self.remote_pubkey {
			// If yes, `server_init`:
			return Ok(self.state.server_receive(client_init.try_into().map_err(|_| { KyberLibError::InvalidLength })?, &pubkey, &self.keypair.secret, &mut self.random)?.into());
		} else {
			// If not, error:
			return Err(KyberLibError::InvalidKey);
		}
	}

	fn client_confirm(&mut self, server_init: &[u8]) -> Result<(), Self::Error> {
		// Final step: propagate errors:
		self.state.client_confirm(server_init.try_into().map_err(|_| { KyberLibError::InvalidLength })?, &self.keypair.secret)?;
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
	bob.set_remote_pubkey(alice.get_local_pubkey().as_slice());
	alice.set_remote_pubkey(bob.get_local_pubkey().as_slice());
	// Alice is the client. Test `client_init`:
	let client_init: Vec<u8> = alice.client_init().expect("Failed to initialize client `alice`!");
	// Bob is the server. Test `server_init`:
	let server_init: Vec<u8> = bob.server_init(&client_init).expect("Failed to initialize server `bob`!");
	// Check it:
	alice.client_confirm(&server_init).expect("Failed to confirm client `alice`!");
}