/*!
	Manages the configuration using the `config` crate.
	We'll be using `tokio`'s `watch` channel to share
	the configuration globally, updated periodically.
	Eventually, I want to use `notify` to only reload
	when the configuration file is changed, but that'll
	have to wait.
*/

const CONFIG_PATH: &str = "~/.qsh/qsh.toml";

use serde::{
	Deserialize,
	self,
};
use toml;
use tokio::{
	fs::{
		self,
		File,
	}, 
	io::{
		AsyncReadExt, 
		AsyncSeekExt,
		SeekFrom,
	},
	sync::watch,
	time::{
		self,
		sleep,
		Duration,
	}
};


// New features go here:
use crate::{
	authenticate::*,
	compress::*,
	crypto::*,
	kex::*,
};


#[derive(Debug, Copy, Clone, Deserialize)]
pub(crate) struct QshConfiguration {
	pub(crate) general: General,
	pub(crate) methods: Methods,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub(crate) struct General {

	// Do we log?
	log: bool,

	// How frequently to check the configuration file, in seconds.
	config_update_interval: u64,
}

/// Methods (from `super`) for various things.
#[derive(Debug, Copy, Clone, Deserialize)]
pub(crate) struct Methods {
	authentication: AuthenticationMethod,
	compression: CompressionMethod,
	crypto: CryptographyMethod,
	key_exchange: KeyExchangeMethod,
}

// Authentication:
#[derive(Debug, Copy, Clone, Deserialize)]
enum AuthenticationMethod {}

// Compression:
#[derive(Debug, Copy, Clone, Deserialize)]
enum CompressionMethod {
	Lz4Flex,
}

// Cryptography:
#[derive(Debug, Copy, Clone, Deserialize)]
enum CryptographyMethod {
	AesGcm,
}

// Key Exchange:
#[derive(Debug, Copy, Clone, Deserialize)]
enum KeyExchangeMethod {
	Kyberlib,
}


impl QshConfiguration {
	pub async fn new() -> Self {
		
		// Open the configuration file:
		let mut file: File = File::open(CONFIG_PATH).await.unwrap();

		// Read it:
		let mut contents: String = String::new();
		file.read_to_string(&mut contents).await.unwrap();

		// Parse it:
		return toml::from_str(&contents).unwrap();
	}
}