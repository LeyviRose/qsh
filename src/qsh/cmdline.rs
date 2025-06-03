use std::{
	net::Ipv6Addr,
};

use clap::{
	Parser,
};


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {

	// IPv6 address to connect to:
	pub host: Ipv6Addr,

	// What application to run (default: `/bin/sh`):
	#[arg(short, long, default_value_t = String::from("/bin/sh"))]
	pub executable: String,

}