use bincode::{Decode, Encode};
use bitflags::bitflags;

#[derive(Encode, Decode)]
pub struct Control {
	
	/// What action will be performed by this packet? See `bitflags` below.
	action: Action,

	/// Any relevant data is placed here.
	data: Vec<u8>,

}

bitflags! {
	/// Specifies which action is being performed by a control packet.
	#[derive(Encode, Decode)]
	pub struct Action: u8 {

		/// Signals the beginning of a new key exchange.
		const kex_init = 0b0000_0001;

		/// Signals a response to a client init.
		const kex_ackn = 0b0000_0010;

		/// Signals a connection termination.
		const close = 0b1000_0000;

	}
}

impl Control {

	pub fn new() -> Self {
		Self { action: Action::empty(), data: vec![] }
	}

	/// Makes a new kex init packet:
	pub fn client_kex_init(&mut self); // TODO: make kex module.
}