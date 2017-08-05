extern crate px;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate xdg;

mod wm;
mod logs;

use std::process;
use std::path::Path;

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
	
	let basedirs = xdg::BaseDirectories::new().unwrap();
	let rcdir = basedirs.get_config_home().join(Path::new("potato"));
	let potatowmrc = basedirs.find_config_file(Path::new("potato/potatowmrc"));

	if let Some(rc) = potatowmrc {
		let setup_script = process::Command::new("sh")
				.stdout(process::Stdio::null())
				.current_dir(rcdir)
				.args(&["-c", rc.to_str().unwrap()])
				.spawn();
		
		match setup_script {
			Ok(_) => { info!("Configured potatowm"); }
			Err(e) => { error!("Couldn't execute potatowmrc: {}", e); },
		}

	} else {
		warn!("Couldn't find potatowmrc in {}", rcdir.to_string_lossy());
	}

	match potato.run() {
		Ok(_) => {
			info!("Exited successfully");
		},
		Err(_) => {
			error!("Error running the window manager");
		}
	};
}