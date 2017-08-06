use px::conn::{Connection, EventLoop};
use px::event::*;

pub mod view;
pub mod node;

use wm::view::{View, ViewMode};
use wm::node::Node;

pub struct WM<'a> {
	conn: &'a Connection,
	events: EventLoop<'a>,
	views: Vec<Box<View>>,
	active_view: usize,
}

impl<'a> WM<'a> {
	pub fn new(conn: &'a Connection, events: EventLoop<'a>) -> WM<'a> {
		let screen_geometry = conn.get_window_geometry(conn.root());

		let views = vec![Box::new(View { mode:ViewMode::Floating,
				nodes:vec![],
				x:0,
				y:0,
				width:screen_geometry.0,
				height:screen_geometry.1,
				padding:10})];

		WM { conn:conn, events:events, views:views, active_view: 0 }
	}

	pub fn run(&mut self) -> Result<(), ()> {
		while let Some(event) = self.events.next() {
			use px::event::Event::*;
			match event {
				MapReqEvent(mapreq) => {
					info!("Got Map Request: {:?}", mapreq);
					self.handle_map(mapreq);
				},
				ConfReqEvent(confreq) => {
					info!("Got Map Request: {:?}", confreq);
					self.handle_configure(confreq);
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

	fn handle_configure(&mut self, req: ConfReq) {
		self.views[self.active_view].add(&self.conn, req);
	}
}