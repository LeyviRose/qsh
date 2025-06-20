use serde::Deserialize;


/// Types of encryption.
#[derive(Deserialize)]
pub enum CryptoTypes {
	AesGcm,
}