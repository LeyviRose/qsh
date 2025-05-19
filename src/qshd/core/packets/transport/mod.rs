//! Contains structures for protocol transport and encryption.

/*!
	There will be two types of packets on this layer:
	1. Control, which handles key exchange, as well as managing the state of the connection (opening, closing).
	2. Data, which moves a packet from a higher level in an encrypted manner.
*/
/*
use bincode::{Decode, Encode};

pub mod control;
pub mod data;

use control::Control;
use data::Data;

#[derive(Encode, Decode)]
pub enum Transport {
	Control(Control),
	Data(Data),
}*/