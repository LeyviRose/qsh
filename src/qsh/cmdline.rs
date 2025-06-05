use std::{
	net::Ipv6Addr,
};

use clap::{
	Parser,
};


#[derive(Debug, Parser)]
#[command(name = "qsh", version, about = "A quantum-safe alternative to SSH.", long_about = None)]
pub struct Args {

	/// IPv6 address to connect to
	pub host: Ipv6Addr,

	/// port to connect to
	pub port: u16,

	/// what application to run (default: `/bin/sh`)
	#[arg(short, long, default_value_t = String::from("/bin/sh"))]
	pub executable: String,

}