use serde::Deserialize;
use std::net::Ipv6Addr;

use super::{
	super::super::super::connection::*,
	crypto::CryptoTypes,
	kex::KexTypes,
};


/// Different types of connections available.
#[derive(Deserialize)]
pub enum ConnectionTypes {
	Tcp,
}

/// Settings for the connection layer.
#[derive(Deserialize)]
pub struct ConnectionConfiguration {

	/// What address to listen on.
	#[serde(default = "default_addr")]
	pub addr: Ipv6Addr,

	/// What port to listen on.
	#[serde(default = "default_port")]
	pub port: u16,

	/// Allowed types of connection.
	#[serde(default = "default_allowed_connection")]
	connection: ConnectionTypes,

	/// Allowed encryption types.
	#[serde(default = "default_allowed_crypto")]
	crypto: CryptoTypes,

	/// Allowed key exchange types.
	#[serde(default = "default_allowed_kex")]
	kex: KexTypes,

}


fn default_allowed_connection() -> ConnectionTypes {
	return ConnectionTypes::Tcp;
}
fn default_allowed_crypto() -> CryptoTypes {
	return CryptoTypes::AesGcm;
}
fn default_allowed_kex() -> KexTypes {
	return KexTypes::Kyberlib;
}
fn default_addr() -> Ipv6Addr {
	return Ipv6Addr::LOCALHOST;
}
fn default_port() -> u16 {
	return 54321;
}