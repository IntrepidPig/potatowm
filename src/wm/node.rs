use px::win::Window;

use wm::view::View;

pub enum Node {
	View(View),
	Window(Window)
}