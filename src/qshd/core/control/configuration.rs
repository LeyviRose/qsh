/*!
	Manages the configuration using the `config` crate.
	We'll be using `tokio`'s `watch` channel to share
	the configuration globally, updated periodically.
	Eventually, I want to use `notify` to only reload
	when the configuration file is changed, but that'll
	have to wait.
*/

use std::{
	net::Ipv6Addr,
	env,
};

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
	session::*,
	channel::*,
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
	#[serde(default)]
	log: bool,

	// How frequently to check the configuration file, in seconds.
	#[serde(default = "default_update_interval")]
	config_update_interval: u64,

	//#[serde(default = "default_listen_address")]
	listen_address: Ipv6Addr,

	//#[serde(default = "default_listen_port")]
	listen_port: u16,

}

/// Methods (from `super`) for various things.
#[derive(Debug, Copy, Clone, Deserialize)]
pub(crate) struct Methods {

	#[serde(default = "default_authentication_method")]
	authentication: AuthenticationMethod,

	#[serde(default = "default_compression_method")]
	compression: CompressionMethod,

	#[serde(default = "default_cryptography_method")]
	crypto: CryptographyMethod,

	#[serde(default = "default_key_exchange_method")]
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
		
		// Locate home directory, and append the correct path:
		let mut config_path: String = env::var("HOME").unwrap();
		config_path.push_str("/.qsh/qsh.toml");

		// Open the configuration file:
		let mut file: File = File::open(config_path).await.unwrap();

		// Read it:
		let mut contents: String = String::new();
		file.read_to_string(&mut contents).await.unwrap();

		// Parse it:
		return toml::from_str(&contents).unwrap();
	}
}


// Defaults:
fn default_update_interval() -> u64 {
	return 30;
}
fn default_authentication_method() -> AuthenticationMethod { todo!() }
fn default_compression_method() -> CompressionMethod {
	return CompressionMethod::Lz4Flex;
}
fn default_cryptography_method() -> CryptographyMethod {
	return CryptographyMethod::AesGcm;
}
fn default_key_exchange_method() -> KeyExchangeMethod {
	return KeyExchangeMethod::Kyberlib;
}