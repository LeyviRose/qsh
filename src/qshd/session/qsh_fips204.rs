/*!
	Implements the default authentication method, CRYSTALS-Dilithium (FIPS-204).
*/

// External dependencies:
use fips204::{
	self,
	ml_dsa_87::{
		self, PrivateKey, PublicKey, PK_LEN, SIG_LEN, SK_LEN
	},
	traits::{SerDes, Signer, Verifier},
};
use rand::SeedableRng;
use tokio::{
	fs::{
		File,
		read_dir,
		ReadDir,
	},
	io::{
		AsyncReadExt,
		AsyncWriteExt
	},
};
use std::{
	collections::HashMap, env, net::Ipv6Addr, os::unix::fs::{MetadataExt, PermissionsExt}, path::PathBuf
};
use rand_chacha::ChaCha20Rng;
use zeroize::{
	ZeroizeOnDrop,
	Zeroizing,
	self,
};
use serde::Deserialize;

// Internal dependencies:
use super::Session;
use crate::{
	crypto,
	kex,
};

const CTX: &'static [u8] = b"qsh";

#[derive(ZeroizeOnDrop)]
pub struct Fips204Authenticator {

	// Local stuff:
	private_key: PrivateKey,
	#[zeroize(skip)]
	public_key: PublicKey,

	#[zeroize(skip)]
	rng: ChaCha20Rng,

	#[zeroize(skip)]
	remote_public_keys: HashMap<Ipv6Addr, PublicKey>,

}

impl Session for Fips204Authenticator {
	type Error = &'static str;
	type Signature = [u8; SIG_LEN];



	async fn new() -> Self {

		// First, let's check if there's already a local key-pair:
		let mut path: PathBuf = PathBuf::from(env::var("HOME").expect("failed to find HOME"));	// Find the home directory.
		path.push(".qsh/fips204");

		// Check the file permissions, etc:
		let (private, public) = if let Ok(mut file_in_question) = File::open(&path).await {
			// Now we check a few things (since the file already exists):
			if file_in_question.metadata().await.expect("failed to read metadata of fips204 key file").mode() & 0o077 == 0 {
				// In this case, group and other have no permission. We're free to read the file!
				let mut sk_buf: [u8; SK_LEN] = [0_u8; SK_LEN];
				let mut pk_buf: [u8; PK_LEN] = [0_u8; PK_LEN];	// Buffer's on the stack, but that's OK, see how I'm using it.
				file_in_question.read_exact(sk_buf.as_mut()).await.expect(&format!("failed to read private key from {:?}, maybe the size is wrong?", &path));
				file_in_question.read_exact(pk_buf.as_mut()).await.expect(&format!("failed to read public key from {:?}, maybe the size is wrong?", &path));

				(PrivateKey::try_from_bytes(sk_buf).expect("failed to deserialize private key"), PublicKey::try_from_bytes(pk_buf).expect("failed to deserialize public key"))
			} else {
				// File exists, but it's got insecure permissions.
				panic!("incorrect file permissions: {:?} shouldn't be accessible to anyone but the owner", &path);
			}
		} else {
			panic!("couldn't find local keys");
		};

		// Now, we need to load the trusted remote keys:
		path.pop();	// Move back up to `~/.qsh`.
		path.push("certs/fips204");	// Change to point to the appropriate public key blob.

		// Validate the file:
		if !path.exists() || !path.is_dir() || path.metadata().unwrap().permissions().mode() & 0o177 != 0 {
			panic!("could not find certs directory, or its permissions were incorrect");
		}
		
		// Loop through it, adding each public key to the hash-table:
		let mut remote_public_keys: HashMap<Ipv6Addr, PublicKey> = HashMap::new();	// For storing the keys.
		let mut paths: ReadDir = read_dir(&path).await.unwrap();	// For reading the directory entries.
		let mut host_buf: [u8; 16] = [0_u8; 16];	// Buffer for holding an IPv6 address.
		let mut cert_buf: [u8; PK_LEN] = [0_u8; PK_LEN];	// Buffer for holding the public key.
		while let Some(entry) = paths.next_entry().await.unwrap() {
			let mut file: File = File::open(entry.path()).await.expect(&format!("failed to open {:?}", entry.path()));	// Open the file for reading.
			file.read_exact(&mut host_buf).await.expect(&format!("failed to read {:?}", entry.path()));	// Read the IPv6 address.
			file.read_exact(&mut cert_buf).await.expect(&format!("failed to read {:?}", entry.path()));	// Read the public key.
			remote_public_keys.insert(	// Insert them into the `HashMap`:
				Ipv6Addr::try_from(host_buf).expect(&format!("IPv6 address at {:?} is not valid", entry.path())),
				PublicKey::try_from_bytes(cert_buf).expect(&format!("public key at {:?} is not valid", entry.path()))
			);
		}

		return Self {
			private_key: private,
			public_key: public,
			rng: ChaCha20Rng::from_entropy(),
			remote_public_keys: remote_public_keys,
		};
	}

	fn sign(&mut self, data: &[u8]) -> Self::Signature {
		return self.private_key.try_sign_with_rng(&mut self.rng, data, CTX).expect("failed to sign data with FIPS-204");
	}

	fn verify(&self, data: &[u8], host: Ipv6Addr, signature: &Self::Signature) -> bool {
		return self.remote_public_keys.get(&host).unwrap().verify(data, signature, CTX);
	}

}

#[test]
fn test_fips204_authenticator() {
	todo!();
}


#[derive(Deserialize)]
pub enum Implementation {
	Fips204,
}

/// Settings for the session layer.
#[derive(Deserialize)]
pub struct SessionConfiguration {

	/// Allowed types of keys.
	#[serde(default = "default_allowed_key")]
	key: Implementation,

	/// Allowed encryption.
	#[serde(default = "default_allowed_crypto")]
	crypto: crypto::Implementation,

	/// Allowed key-exchange.
	#[serde(default = "default_allowed_kex")]
	kex: kex::Implementation,

}


fn default_allowed_crypto() -> crypto::Implementation {
	return crypto::Implementation::AesGcm;
}
fn default_allowed_kex() -> kex::Implementation {
	return kex::Implementation::Kyberlib;
}
fn default_allowed_key() -> Implementation {
	return Implementation::Fips204;
}