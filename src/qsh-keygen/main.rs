/*
	For generating and managing keys.
*/

use fips204::{
	ml_dsa_87::{
		PrivateKey,
		PublicKey,
		SK_LEN,
		PK_LEN,
		self,
	},
	traits::SerDes,
};
use rand_chacha::{
	ChaCha20Rng,
	rand_core::SeedableRng,
};
use zeroize::Zeroizing;
use std::{
	env, fs::{read, set_permissions, write, File, Permissions}, io::{Read, Seek, Write, SeekFrom}, net::Ipv6Addr, os::unix::fs::{OpenOptionsExt, PermissionsExt}, path::{
		Path,
		PathBuf,
	}
};
use clap::{
	Parser,
	Subcommand,
};


#[derive(Debug, Parser)]
#[command(name = "qsh-keygen", version, about = "Generates and manages QSH keys.", long_about = None)]
struct Args {

	#[command(subcommand)]
	operation: Operation,

}

#[derive(Debug, Subcommand)]
enum Operation {

	/// create a new keypair
	New {

		#[command(subcommand)]
		key_type: KeyType,

	},

	/// add a remote public key to the key collection
	Add {

		#[command(subcommand)]
		key_type: KeyType,

		name: String,

		path: String,

		host: Ipv6Addr,

	},

	/// delete a keypair
	Del {

		name: String,

	},

	/// remove a remote public key
	Rem {

		name: String,

	},

	/// export public key:
	Exp {

		#[command(subcommand)]
		key_type: KeyType,

		/// file to export to
		export_to: String,

	}

}

/// The type of key in question.
#[derive(Debug, Subcommand)]
enum KeyType {
	Fips204,
}


fn main() {
	let args: Args = Args::parse();

	// Let's find the qsh directory:
	let mut qsh_directory: PathBuf = PathBuf::from(env::var("HOME").unwrap());
	qsh_directory.push(".qsh");

	match args.operation {
		Operation::New {key_type} => {
			new(key_type, qsh_directory.clone());
		},
		Operation::Add {key_type, name, path, host} => {
			add(key_type, qsh_directory.join("certs").join(name), &PathBuf::from(path), &host);
		},
		Operation::Del {name} => todo!(),
		Operation::Rem {name} => todo!(),
		Operation::Exp {key_type, export_to} => {
			exp(key_type, &PathBuf::from(export_to), &qsh_directory);
		},
	}
}


/// Makes a new keypair.
fn new(key_type: KeyType, mut qsh_dir: PathBuf) {
	let mut random: ChaCha20Rng = ChaCha20Rng::from_entropy();

	match key_type {
		KeyType::Fips204 => {

			qsh_dir.push("fips204");
			let (p, s) = ml_dsa_87::try_keygen_with_rng(&mut random).expect("failed to generate new keypair");

			let mut new_file: File = File::options().read(false).write(true).create(true).open(&qsh_dir).unwrap();	// Create file, read-only, inaccessible to others.
			new_file.write_all(&s.clone().into_bytes()).expect("failed to write new private key to file");
			new_file.write_all(&p.clone().into_bytes()).expect("failed to write new public key to file");

		},
	}

	// Set the right file permissions:
	set_permissions(&qsh_dir, Permissions::from_mode(0o400)).unwrap();
}

/// Adds a remote public key to the key collection.
fn add(key_type: KeyType, write_path: PathBuf, read_path: &Path, host: &Ipv6Addr) {
	match key_type {
		KeyType::Fips204 => {

			// First, validate the key:
			let mut new_key: [u8; PK_LEN] = [0_u8; PK_LEN];
			let mut i_file: File = File::open(read_path).unwrap();
			i_file.read_exact(&mut new_key).unwrap();

			if let Ok(_) = PublicKey::try_from_bytes(new_key) {
				// If valid:
				let mut o_file: File = File::create(&write_path).unwrap();
				o_file.write_all(&host.octets()).unwrap();
				o_file.write_all(&new_key).unwrap();
			} else {
				// Trying to add an invalid key:
				panic!("key invalid");
			}
		},
	}

	// Set file permissions:
	set_permissions(&write_path, Permissions::from_mode(0o400)).unwrap();
}

/// Deletes a keypair.
fn del() {
	todo!();
}

/// Removes a remote public key from the key collection.
fn rem() {
	todo!();
}

/// Exports the public key.
fn exp(key_type: KeyType, export_to: &Path, qsh_dir: &Path) {
	match key_type {
		KeyType::Fips204 => {
			let mut buf: [u8; PK_LEN] = [0_u8; PK_LEN];
			let mut in_key: File = File::open(qsh_dir.join("fips204")).expect("couldn't open key file");
			in_key.seek(SeekFrom::Start(SK_LEN as u64)).unwrap();
			in_key.read_exact(&mut buf).unwrap();
			write(export_to, &buf).unwrap();
		},
	}
}