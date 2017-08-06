use px::conn::Connection;
use px::event::ConfReq;

use wm::node::Node;

pub struct View {
	pub mode: ViewMode,
	pub nodes: Vec<Node>,
	pub x: i16,
	pub y: i16,
	pub width: u16,
	pub height: u16,
	pub padding: u16,
}

pub enum ViewMode {
	Floating,
	Tiled,
	Tabbed,
	Fullscreen,
}

impl View {
	pub fn add(&mut self, conn: &Connection, req: ConfReq) {
		conn.configure_window(&req.window, req.x, req.y, req.width, req.height);
		self.nodes.push(Node::Window(req.window));		
	}

	pub fn layout(&self, conn: &Connection) {
		match &self.mode {
			Foating => {
				for (i, node) in self.nodes.iter().enumerate() {
					
				}
			}
		}
	}
}