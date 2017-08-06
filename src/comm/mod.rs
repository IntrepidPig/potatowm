use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use serde_json;
use serde_json::{Value, Error};

use wm::ctrl;
use wm::WM;
use wm::ctrl::Request;

pub fn run(requests: Arc<Mutex<VecDeque<Request>>>) -> Result<(), ()> {
	let listener = TcpListener::bind("127.0.0.1:8000").expect("Couldn't bind to local address"); // TODO make this configurable

	for stream in listener.incoming() {
		let mut stream = match stream {
			Ok(stream) => {
				stream
			},
			Err(_) => {
				warn!("Error creating TcpClient");
				continue;
			}
		};

		let mut cmd = String::new();
		match stream.read_to_string(&mut cmd) {
			Err(_) => {
				warn!("Error communicating with TcpClient");
			}
			_ => {},
		}

		info!("Got icp command: {}", cmd.trim());

		let request: Request = match serde_json::from_str(&cmd) {
			Ok(req) => req,
			Err(_) => {
				warn!("Couldn't parse request json ({})", cmd.trim());
				continue;
			}
		};

		requests.lock().unwrap().push_back(request);
	}

	Ok(())
}