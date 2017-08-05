use px::conn::{Connection, EventLoop};
use px::event::*;

pub struct WM<'a> {
	conn: &'a Connection,
	events: EventLoop<'a>,
}

impl<'a> WM<'a> {
	pub fn new(conn: &'a Connection, events: EventLoop<'a>) -> WM<'a> {
		WM { conn:conn, events:events }
	}

	pub fn run(&mut self) -> Result<(), ()> {
		for event in &self.events {
			use px::event::Event::*;
			match event {
				MapReqEvent(mapreq) => {
					info!("Got Map Request: {:?}", mapreq);
					self.handle_map(mapreq);
				},
				ConfReqEvent(confreq) => {
					info!("Got Map Request: {:?}", confreq);
				},
				_ => {
					warn!("Got unknown event: {:?}", event);
				}
			};
		}

		Ok(())
	}

	fn handle_map(&self, req: MapReq) {
		self.conn.map_window(req.window);
	}
}