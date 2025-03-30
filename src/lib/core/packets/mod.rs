//! Contains structures representing packets in the protocol.

/*
	The way things are layed out:
	transport {
		control | auth {
			control | session {
				control | stream {
					control | payload;
				}
			}
		}
	}
	Where "control" is a control packet for the relevant layer.
*/

pub mod transport;
pub mod auth;
pub mod session;
pub mod stream;


// Re-export relevant thigs here: