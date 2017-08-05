extern crate px;

mod wm;

use wm::WM;

fn main() {
	let conn = px::conn::Connection::new().unwrap();
	let events = px::conn::EventLoop::new(&conn).unwrap();
	let mut potato = WM::new(&conn, events);
	
	match potato.run() {
		Ok(_) => {
			println!("Exited successfully");
		},
		Err(_) => {
			eprintln!("Error running the window manager");
		}
	};
}