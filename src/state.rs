use glium::glutin::event_loop::EventLoop;
use glium::glutin::ContextBuilder;
use glium::Display;
use glium::glutin;
use glutin::window::*;

pub struct State_ {
  window: Window,
  display: Display<>,
}
impl State_ {
	pub fn window(&mut self) -> &mut Window {
		&mut self.window
	}
	pub fn display(&mut self) -> &mut Display<> {
		&mut self.display
	}
    pub fn new(window_builder: WindowBuilder, event_loop: &EventLoop<()>) -> Self {
        let window = window_builder.clone().build(event_loop).expect("WINDOW BUILD ERROR");
        let context_builder = ContextBuilder::new();
        let display = Display::new(window_builder, context_builder, event_loop).expect("DISPLAY BUILD ERROR");
        Self { window, display }
    }
    pub fn get_size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }
}