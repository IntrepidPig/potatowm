extern crate px;
#[macro_use]
extern crate log;
extern crate log4rs;

mod wm;
mod logs;

use wm::WM;

fn main() {
	#[allow(unused_variables)]
	let handle = match logs::init_logging() {
		Ok(handle) => handle,
		Err(e) => {
			error!("Error initializing the logger");
			return;
		}
	};

	info!("Starting PotatoWM");
	info!("If you're reading this, logging has initialized successfully");

	let conn = px::conn::Connection::new().unwrap();
	let events = px::conn::EventLoop::new(&conn).unwrap();
	let mut potato = WM::new(&conn, events);
	
	match potato.run() {
		Ok(_) => {
			info!("Exited successfully");
		},
		Err(_) => {
			error!("Error running the window manager");
		}
	};
}