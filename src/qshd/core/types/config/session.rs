use serde::Deserialize;

use super::{
	crypto::CryptoTypes,
	kex::KexTypes,
};


#[derive(Deserialize)]
pub enum KeyTypes {
	Fips204,
}

/// Settings for the session layer.
#[derive(Deserialize)]
pub struct SessionConfiguration {

	/// Allowed types of keys.
	#[serde(default = "default_allowed_key")]
	key: KeyTypes,

	/// Allowed encryption.
	#[serde(default = "default_allowed_crypto")]
	crypto: CryptoTypes,

	/// Allowed key-exchange.
	#[serde(default = "default_allowed_kex")]
	kex: KexTypes,

}


fn default_allowed_crypto() -> CryptoTypes {
	return CryptoTypes::AesGcm;
}
fn default_allowed_kex() -> KexTypes {
	return KexTypes::Kyberlib;
}
fn default_allowed_key() -> KeyTypes {
	return KeyTypes::Fips204;
}