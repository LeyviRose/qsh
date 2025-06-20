use serde::Deserialize;


/// Types of key exchange.
#[derive(Deserialize)]
pub enum KexTypes {
	Kyberlib,
}