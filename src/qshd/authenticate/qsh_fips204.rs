/*!
	Implements the default authentication method, CRYSTALS-Dilithium (FIPS-204).
	TODO: Improve use of `zeroize`.
*/

// External dependencies:
use fips204::{
	self,
	ml_dsa_87::{
		self, PrivateKey, PublicKey, PK_LEN, SIG_LEN, SK_LEN
	},
	traits::{SerDes, Signer, Verifier},
};
use tokio::{
	fs::{
		File,
		OpenOptions,
	},
	io::{
		AsyncReadExt,
		AsyncWriteExt
	},
};
use std::{
	collections::HashMap, env, net::Ipv6Addr, os::unix::fs::MetadataExt, path::PathBuf
};
use rand_chacha::{
	ChaCha20Rng,
	rand_core::SeedableRng,
};
use zeroize::{
	ZeroizeOnDrop,
	Zeroizing,
	self,
};

// Internal dependencies:
use super::Authenticator;

const CTX: &'static [u8] = b"qsh";

#[derive(ZeroizeOnDrop)]
pub struct Fips204Authenticator {

	// Local stuff:
	private_key: PrivateKey,
	public_key: PublicKey,

	#[zeroize(skip)]
	rng: ChaCha20Rng,

	#[zeroize(skip)]
	remote_public_keys: HashMap<Ipv6Addr, PublicKey>,

}

impl Authenticator for Fips204Authenticator {
	type Error = &'static str;
	type Signature = [u8; SIG_LEN];


	async fn new() -> Self {

		let mut random: ChaCha20Rng = ChaCha20Rng::from_entropy();	// Instantiate new CSPRNG for later.

		// First, let's check if there's already a local key-pair:
		let mut path: PathBuf = PathBuf::from(env::var("HOME").expect("failed to find HOME"));	// Find the home directory.
		path.push("/.qsh/local_keys/fips204");	// Append this directory.

		// Check the file permissions, etc:
		let (private, public) = if let Ok(mut file_in_question) = File::open(&path).await {
			// Now we check a few things (since the file already exists):
			if file_in_question.metadata().await.expect("failed to read metadata of fips204 key file").mode() & 0o077 == 0 {
				// In this case, group and other have no permission. We're free to read the file!
				let mut sk_buf: Zeroizing<[u8; SK_LEN]> = Zeroizing::new([0_u8; SK_LEN]);
				let mut pk_buf: Zeroizing<[u8; PK_LEN]> = Zeroizing::new([0_u8; PK_LEN]);	// Buffer's on the stack, but that's OK, see how I'm using it.
				file_in_question.read_exact(sk_buf.as_mut()).await.expect(&format!("failed to read private key from {:?}, maybe the size is wrong?", &path));
				file_in_question.read_exact(pk_buf.as_mut()).await.expect(&format!("failed to read public key from {:?}, maybe the size is wrong?", &path));

				(PrivateKey::try_from_bytes(*sk_buf).expect("failed to deserialize private key"), PublicKey::try_from_bytes(*pk_buf).expect("failed to deserialize public key"))
			} else {
				// File exists, but it's got insecure permissions.
				panic!("incorrect file permissions: {:?} shouldn't be accessible to anyone but the owner", &path);
			}
		} else {
			panic!("couldn't find local keys");
		};

		return Self {
			private_key: private,
			public_key: public,
			rng: random,
		};
	}

	fn sign(&mut self, data: &[u8]) -> Self::Signature {
		return self.private_key.try_sign_with_rng(&mut self.rng, data, CTX).expect("failed to sign data with FIPS-204");
	}

	fn verify(&self, data: &[u8], host: Ipv6Addr, signature: &Self::Signature) -> bool {
		return self.remote_public_keys.get(&host).unwrap().verify(data, signature, CTX);
	}

}