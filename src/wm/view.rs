use px::conn::Connection;
use px::event::ConfReq;
use px::win::Window;

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
	Tiling,
	Tabbed,
	Fullscreen,
}

impl View {
	pub fn add(&mut self, conn: &Connection, win: Window) {
		
		self.nodes.push(Node::Window(win));		
	}

	pub fn layout(&self, conn: &Connection) {
		match &self.mode {
			Foating => {
				for (i, node) in self.nodes.iter().enumerate() {
					
				}
			},
			Tiling => {
				for (i, node) in self.nodes.iter().enumerate() {
					
				}
			}
		}
	}
}