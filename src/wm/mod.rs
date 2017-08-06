use px::conn::{Connection, EventLoop};
use px::event::*;

pub mod view;
pub mod node;
pub mod ctrl;

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use wm::ctrl::Request;
use wm::view::{View, ViewMode};
use wm::node::Node;

pub struct WM<'a> {
	conn: &'a Connection,
	events: EventLoop<'a>,
	views: Vec<Box<View>>,
	active_view: usize,
	pub requests: Arc<Mutex<VecDeque<Request>>>,
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
				gap:10})];
		
		let requests: VecDeque<Request> = VecDeque::new();

		WM { conn:conn, events: events, views:views, active_view: 0, requests: Arc::new(Mutex::new(requests)) }
	}

	pub fn run(&mut self) -> Result<(), ()> {
		loop {
			if let Some(event) = self.events.next() {
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
			else if let Some(cmd) = self.requests.lock().unwrap().pop_front() {
				self.handle_cmd(cmd);
			}
		}

		Ok(())
	}

	pub fn active_view(&mut self) -> &mut View {
		&mut self.views[self.active_view]
	}

	fn handle_map(&self, req: MapReq) {
		self.conn.map_window(req.window);
	}

	fn handle_configure(&mut self, req: ConfReq) {
		self.conn.configure_window(&req.window, req.x, req.y, req.width, req.height);

		let active_view = self.active_view();
		active_view.add(self.conn, req.window);
	}

	fn handle_cmd(&mut self, cmd: Request) {

	}
}