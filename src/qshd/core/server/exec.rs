use tokio::{
	io::{AsyncReadExt, AsyncWriteExt}, process, sync::mpsc, task
};
use std::process::Stdio;

const BUFSIZE: usize = 4096;


/// Run `command`. `i` links to `stdin`, `o` to `stdout`, and `e` to `stderr`.
async fn start(command: &str, mut i: mpsc::Receiver<Vec<u8>>, o: mpsc::Sender<Vec<u8>>, e: mpsc::Sender<Vec<u8>>) {

	// Process handle:
	let mut handle: process::Child = process::Command::new(command).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();

	// Let's take the I/O streams:
	let mut stdin: process::ChildStdin = handle.stdin.take().unwrap();
	let mut stdout: process::ChildStdout = handle.stdout.take().unwrap();
	let mut stderr: process::ChildStderr = handle.stderr.take().unwrap();

	// Start the tasks that transfer between the channels and the streams:
	task::spawn(async move {
		// While there might be data in the channel:
		while let Some(data) = i.recv().await {
			// Write that data to `stdin`.
			stdin.write_all(&data).await.unwrap();
		}
	});
	task::spawn(async move {
		let mut buf: Vec<u8> = Vec::with_capacity(BUFSIZE);
		// While `stdout` is open (no EOF):
		while stdout.read(&mut buf).await.unwrap() != 0 {
			// Read and send data:
			o.send(buf.clone()).await.unwrap();
		}
	});
	task::spawn(async move {
		let mut buf: Vec<u8> = Vec::with_capacity(BUFSIZE);
		// While `stderr` is open (no EOF):
		while stderr.read(&mut buf).await.unwrap() != 0 {
			// Read and send data:
			e.send(buf.clone()).await.unwrap();
		}
	});
}