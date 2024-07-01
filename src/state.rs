use glium::{backend::glutin::SimpleWindowBuilder, glutin::surface::WindowSurface, Display};
use winit::{event_loop::EventLoop, window::Window};


pub struct State_ {
  window: Window,
  display: Display<WindowSurface>,
}
impl State_ {
	pub fn window(&mut self) -> &mut Window {
		&mut self.window
	}
	pub fn display(&mut self) -> &mut Display<WindowSurface> {
		&mut self.display
	}
    pub fn new(window_builder: SimpleWindowBuilder, event_loop: &EventLoop<()>) -> Self {
        let (window, display) = window_builder.build(event_loop);
        Self { window, display }
    }
    pub fn get_size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }
}