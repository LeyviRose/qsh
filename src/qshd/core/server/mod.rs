/*!
	Thing that behaves like a server.
*/

use tokio::{
	task,
	sync::mpsc,
};


pub mod config;
mod exec;
use config::ServerConfiguration;


pub struct Server {} impl Server {

	pub async fn start() {

		// First, load the configuration, and check if the server is meant to run:
		if let Some(configuration) = ServerConfiguration::load().await {
			todo!()
		} else {
			// If the server isn't meant to run (it's not configured to accept any clients):
			eprintln!("Server not configured, skipping startup.");
			return;
		}
	}
}