use clap::Parser;
use std::{
	env,
	path,
};

mod daemon;
mod cmdline;

use cmdline::Args;
use daemon::Daemon;

#[tokio::main]
async fn main() {

	// First, command-line:
	let arguments: Args = Args::parse();

	// Now, find XDG_RUNTIME_DIR:
	let mut socketpath: path::PathBuf = path::PathBuf::from(env::var("XDG_RUNTIME_DIR").expect("failed to find XDG_RUNTIME_DIR"));
	socketpath.push("/qshd.socket");	// We're calling the socket "qshd.socket".

	// Connect:
	let service: Daemon = Daemon::new(socketpath, arguments.host, arguments.port, arguments.executable.into()).await.expect("failed to connect to qshd");
}