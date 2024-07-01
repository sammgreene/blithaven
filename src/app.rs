use std::collections::HashSet;

use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::event::{ElementState, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::{event::{Event, WindowEvent}, window::WindowBuilder};

use crate::{batches::*, state::State_, vertex::Vertex};

pub struct AppBuilder {
  window_builder: SimpleWindowBuilder
}
impl AppBuilder {
	pub fn new() -> Self {
		Self { window_builder: SimpleWindowBuilder::new().set_window_builder(WindowBuilder::new().with_resizable(false)) }
	}
	pub fn set_size(self, width: u32, height: u32) -> Self {
		Self { window_builder: self.window_builder.with_inner_size(width, height) }
	}
	pub fn run<F>(self, mut input_code: F) ->! where F: 'static + FnMut(f64) {
		let event_loop = EventLoop::new();
		//let mut app = App_::new(self.window_builder, &event_loop);
		crate::app::init_app(self.window_builder, &event_loop);
		let app = get_app();

		crate::version();

		let mut time = std::time::Instant::now();
		// let mut next_frame = std::time::Instant::now();

		event_loop.run(move | event, _, control_flow | {
			input_code(time.elapsed().as_secs_f64());
			time = std::time::Instant::now();


			// Per Frame
			app.draw();
			app.key_events.clear();
			app.key_releases.clear();
			app.key_presses.clear();

			match event {
				Event::WindowEvent {ref event, window_id } if window_id == app.state().window().id() => {
					match event {
						WindowEvent::CloseRequested => {
							println!("Exiting...");
							*control_flow = ControlFlow::Exit;
						}
						WindowEvent::KeyboardInput { input, .. } => {
							if input.virtual_keycode.is_some() {
								let key = input.virtual_keycode.unwrap();
								app.key_events.insert(key);
								match input.state {
									ElementState::Pressed => {
										app.pressed_keys.insert(key);
										app.key_presses.insert(key);
									},
									ElementState::Released => {
										app.pressed_keys.remove(&key);
										app.key_releases.insert(key);
									}
								}
							}
						},
						WindowEvent::MouseInput { button, state, .. } => {
						},
						WindowEvent::CursorMoved { position, .. } => {
						}

						_ => ()
					} // END of match event: WindowEvent
				}

				_ => ()
			} // END of match event: Event
		}); // END of event_loop.run()
	}
}

pub struct App_ {
  state: State_,
  batches: Vec<Batch>,
	texture_batches: Vec<TextureBatch>,
  quad_count: u32,

	pressed_keys: std::collections::HashSet<VirtualKeyCode>,
	key_events: std::collections::HashSet<VirtualKeyCode>,
	key_presses: std::collections::HashSet<VirtualKeyCode>,
	key_releases: std::collections::HashSet<VirtualKeyCode>
}
impl App_ {
	pub fn state(&mut self) -> &mut State_ {
		&mut self.state
	}
	pub fn pressed_keys(&self) -> HashSet<VirtualKeyCode> {
		self.pressed_keys.clone()
	}
	pub fn key_presses(&self) -> HashSet<VirtualKeyCode> {
		self.key_presses.clone()
	}
	pub fn key_events(&self) -> HashSet<VirtualKeyCode> {
		self.key_events.clone()
	}
	fn new(window_builder: SimpleWindowBuilder, event_loop: &EventLoop<()>) -> Self {
		let state = State_::new(window_builder, event_loop);
		Self { state, batches: vec![], texture_batches: vec![], quad_count: 0, pressed_keys: HashSet::new(), key_events: HashSet::new(), key_presses: HashSet::new(), key_releases: HashSet::new() }
	}
	fn add_batch(&mut self) {
		let batch = Batch::new(&self.state.display());
		self.batches.push(batch);
	}
	fn add_texture_batch(&mut self, texture_path: String) {
		let batch = TextureBatch::new(&self.state.display(), texture_path);
		if batch.is_some() {
			self.texture_batches.push(batch.unwrap());
		}
	}
	fn full_batches(&self) -> bool {
		for batch in self.batches.iter() {
			if !batch.is_full() {
				return false
			}
		}
		true
	}
	fn full_texture_batches(&self, path: &str) -> bool {
		for batch in self.texture_batches.iter() {
			if !batch.is_full() && batch.path() == path {
				return false
			}
		}
		true
	}

	pub fn add_quad(&mut self, pos: [i32; 2], w: i32, h: i32, color: [f32; 4], style: f32) {
		let window_size = self.state.get_size();

		let position = [(pos[0] as f32 / window_size.1 as f32) - (window_size.0 as f32 / window_size.1 as f32), (-pos[1] as f32 / window_size.1 as f32) + 1.0];
		let width = w as f32 / window_size.1 as f32;
		let height = -h as f32 / window_size.1 as f32;

		if self.full_batches() {
			self.add_batch();
		}

		for batch in self.batches.iter_mut() {
			if !batch.is_full() {
				let index_offset = batch.vertecies().len() as u32;
				self.quad_count += 1;
				let mut verts = vec![
					Vertex { position: [position[0]        , position[1]         , self.quad_count as f32], color, uv: [0.0,0.0], style: style as f32 },
					Vertex { position: [position[0]        , position[1] + height, self.quad_count as f32], color, uv: [0.0,1.0], style: style as f32 },
					Vertex { position: [position[0] + width, position[1]         , self.quad_count as f32], color, uv: [1.0,0.0], style: style as f32 },
					Vertex { position: [position[0] + width, position[1] + height, self.quad_count as f32], color, uv: [1.0,1.0], style: style as f32 }
				];
				batch.vertecies().append(&mut verts);

				let mut inds = vec![
					index_offset + 0, index_offset + 1, index_offset + 2,
					index_offset + 2, index_offset + 1, index_offset + 3
				];
				batch.indecies().append(&mut inds);
				return
			}
		}
	}
	pub fn add_texture_quad(&mut self, pos: [i32; 2], w: i32, h: Option<i32>, color: [f32; 4], texture_path: String) {

		let mut style = 2.0;
		if texture_path == "rectangle" {
			style = 0.0;
		}
		if texture_path == "circle" {
			style = 1.0;
		}
		
		if self.full_texture_batches(&texture_path) {
			self.add_texture_batch(texture_path.clone());
		}

		for batch in self.texture_batches.iter_mut() {
			if !batch.is_full() && batch.path() == &texture_path {
				let mut height = h;
				if h.is_none() {
					let dims = batch.texture().dimensions();
					let ratio = dims.1 as f32 / dims.0 as f32;
					
					height = Some( (w as f32 * (ratio) ) as i32);
				}
				let window_size = self.state.get_size();

				let position = [(pos[0] as f32 / window_size.1 as f32) - (window_size.0 as f32 / window_size.1 as f32), (-pos[1] as f32 / window_size.1 as f32) + 1.0];
				let width = w as f32 / window_size.1 as f32;
				let height = -(height.unwrap()) as f32 / window_size.1 as f32;

				let index_offset = batch.vertecies().len() as u32;
				self.quad_count += 1;
				let mut verts = vec![
					Vertex { position: [position[0]        , position[1]         , self.quad_count as f32], color, uv: [0.0,1.0], style },
					Vertex { position: [position[0]        , position[1] + height, self.quad_count as f32], color, uv: [0.0,0.0], style },
					Vertex { position: [position[0] + width, position[1]         , self.quad_count as f32], color, uv: [1.0,1.0], style },
					Vertex { position: [position[0] + width, position[1] + height, self.quad_count as f32], color, uv: [1.0,0.0], style }
				];
				batch.vertecies().append(&mut verts);

				let mut inds = vec![
					index_offset + 0, index_offset + 1, index_offset + 2,
					index_offset + 2, index_offset + 1, index_offset + 3
				];
				batch.indecies().append(&mut inds);
				return
			}
		}
	}

	pub fn draw(&mut self) {
		let mut target = self.state.display().draw();
		let (width, height) = target.get_dimensions();
		target.clear_color_and_depth((0.2,0.2,0.2,1.0), 1.0);
		for batch in self.batches.iter_mut() {
			batch.draw(&self.state.display(), &mut target, height as f32 / width as f32);
		}
		for tex_batch in self.texture_batches.iter_mut() {
			tex_batch.draw(&self.state.display(), &mut target, height as f32 / width as f32);
		}
		self.quad_count = 0;
		target.finish().unwrap();
	}
}

pub static mut APP: Option<App_> = None;
pub fn init_app(window_builder: SimpleWindowBuilder, event_loop: &EventLoop<()>) { unsafe { APP= Some(App_::new(window_builder, &event_loop)) } }
pub fn get_app() -> &'static mut App_ { unsafe { APP.as_mut().unwrap() } }