

use glium::texture::SrgbTexture2d;
use glium::{Blend, Frame, IndexBuffer, Surface, VertexBuffer};
use glium::{Display, Program};

use crate::{vertex::Vertex, shaders};

pub struct Batch {
  vertecies: Vec<Vertex>,
  indecies: Vec<u32>,
  program: Program,

  max_quads: usize
}

impl Batch {
	pub fn vertecies(&mut self) -> &mut Vec<Vertex> {
		&mut self.vertecies
	}
	pub fn indecies(&mut self) -> &mut Vec<u32> {
		&mut self.indecies
	}
	pub fn new(facade: &Display<>) -> Self {
		let vertecies = vec![];
		let indecies = vec![];

		let vertex_shader = shaders::BASE_VERTEX_SHADER;
		let fragment_shader = shaders::BASE_FRAG_SHADER;

		let program = glium::Program::from_source(facade, vertex_shader, fragment_shader, None).unwrap();
		
		Self {
			vertecies,
			indecies,
			program,

			max_quads: 20000
		}
	}

	pub fn draw(&mut self, display: &Display<>, target: &mut Frame, aspect_ratio: f32) {
		let uniforms = glium::uniform! {
			matrix: [
				[aspect_ratio,0.0,0.0,0.0],
				[0.0,1.0,0.0,0.0],
				[0.0,0.0,1.0,0.0],
				[0.0,0.0,0.0,1.0f32]
			]
		};
		let params = glium::DrawParameters {
			depth: glium::Depth {
			    test: glium::draw_parameters::DepthTest::IfLessOrEqual,
			    write: true,
			    .. Default::default()
			},
			blend: Blend::alpha_blending(),
			.. Default::default()
		};
		let vertex_buffer = VertexBuffer::new(display, &self.vertecies).unwrap();
		let index_buffer = IndexBuffer::new(
			display,
			glium::index::PrimitiveType::TrianglesList,
			&self.indecies).unwrap();


		target.draw(&vertex_buffer, &index_buffer, &self.program, &uniforms,
					&params).unwrap();

		self.vertecies = vec![];
		self.indecies = vec![];
	}
	pub fn is_full(&self) -> bool {
		self.vertecies.len() >= self.max_quads * 4
	}
}

#[allow(dead_code)]
pub struct TextureBatch {
  path: String,
  vertecies: Vec<Vertex>,
  indecies: Vec<u32>,
  program: Program,

  texture: glium::texture::SrgbTexture2d,
  max_quads: usize
}
#[allow(dead_code)]
impl TextureBatch {
	pub fn vertecies(&mut self) -> &mut Vec<Vertex> {
		&mut self.vertecies
	}
	pub fn indecies(&mut self) -> &mut Vec<u32> {
		&mut self.indecies
	}
	pub fn path(&self) -> &String {
		&self.path
	}
	pub fn texture(&self) -> &glium::texture::SrgbTexture2d {
		&self.texture
	}
	pub fn new(facade: &Display<>, path_or_char: String) -> Option<Self>  {
		let last_five = {
			let mut split_pos: usize = 0;
			if path_or_char.len() >= 4 {
				split_pos = path_or_char.char_indices().nth_back(3).unwrap().0;
			}
			&path_or_char[split_pos..]
		};
		if last_five == ".png" {
			let texture = new_texture(&path_or_char, facade);
			
			return Some(Self::from_texture(facade, texture, path_or_char));
		}
		if path_or_char.is_ascii() && path_or_char.len() == 1 {
			for letter in crate::font::BITMAPFONT {
				if letter.0 == path_or_char.chars().nth(0).unwrap() {
					let mut letter_image: [u8; 64 * 4] = [0; 256];
					let letter_data = letter.1;
					for i in 0..8 {
						for j in 0..8 {
							let shifted_byte = letter_data[i] >> j;
							let bit = shifted_byte & 1;
							if bit == 1 {
									letter_image[(i * 8 + j) * 4 + 0] = 255;
									letter_image[(i * 8 + j) * 4 + 1] = 255;
									letter_image[(i * 8 + j) * 4 + 2] = 255;
									letter_image[(i * 8 + j) * 4 + 3] = 255;
							}
						}
					}
					let image: glium::texture::RawImage2d<'_, u8> = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &letter_image,
            (8,8));
        	let texture = glium::texture::SrgbTexture2d::new(facade, image).unwrap();
					return Some(Self::from_texture(facade, texture, path_or_char))
				}
			}
			None
		}
		else {
			if path_or_char == "circle" || path_or_char == "rectangle" {
				let image: glium::texture::RawImage2d<'_, u8> = glium::texture::RawImage2d::from_raw_rgba_reversed(
					&[0,0,0,0],
					(1,1));
				let texture = glium::texture::SrgbTexture2d::new(facade, image).unwrap();
				return Some(Self::from_texture(facade, texture, path_or_char))
			}
			println!("ERROR: No png file at path specified: {}", path_or_char);
			None
		}

	}
	fn from_texture(facade: &Display<>, texture: SrgbTexture2d, path: String) -> Self {
		let vertecies = vec![];
		let indecies = vec![];

		let vertex_shader = shaders::TEX_VERTEX_SHADER;
		let frag_shader = shaders::TEX_FRAG_SHADER;

		let program = glium::Program::from_source(facade, vertex_shader, frag_shader, None).unwrap();

		Self {
			path,
			vertecies,
			indecies,
			program,
			
			texture,
			max_quads: 20000
		}
	}

  pub fn draw(&mut self, display: &Display<>, target: &mut Frame, aspect_ratio: f32) {

		let behavior = glium::uniforms::SamplerBehavior {
            minify_filter: glium::uniforms::MinifySamplerFilter::Linear,
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            ..Default::default()
    };

		let uniforms = glium::uniform! {
			matrix: [
				[aspect_ratio,0.0,0.0,0.0],
				[0.0,1.0,0.0,0.0],
				[0.0,0.0,1.0,0.0],
				[0.0,0.0,0.0,1.0f32]
			],
            tex: glium::uniforms::Sampler(&self.texture, behavior),
		};

		let params = glium::DrawParameters {
			// depth: glium::Depth {
			//     test: glium::draw_parameters::DepthTest::IfLessOrEqual,
			//     write: true,
			//     .. Default::default()
			// },
			blend: Blend::alpha_blending(),
			.. Default::default()
		};
		
		let vertex_buffer = VertexBuffer::new(display, &self.vertecies).unwrap();
		let index_buffer = IndexBuffer::new(
			display,
			glium::index::PrimitiveType::TrianglesList,
			&self.indecies
		).unwrap();
	
		// should really only be called on the very first frame or something. This just spams and is hard to read
		// for i in 0 .. self.vertecies.len() / 4 {
		// 	println!("{:?} - z: {:?}", self.path, self.vertecies[i * 4].position[2]);
		// }
		target.draw(&vertex_buffer, &index_buffer, &self.program, &uniforms,
					&params
		).unwrap();
		
		self.vertecies = vec![];
		self.indecies = vec![];
  }
	pub fn is_full(&self) -> bool {
		self.vertecies.len() >= self.max_quads * 4
	}
}

fn new_texture(path: &str, display: &Display<>) -> glium::texture::SrgbTexture2d {
	use std::fs::File;
	use std::io::Read;
	let f = File::open(path).unwrap();
	let mut reader = std::io::BufReader::new(f);
	let mut buffer = Vec::new();
	reader.read_to_end(&mut buffer).unwrap();

	let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::load(std::io::Cursor::new(&buffer),
		image::ImageFormat::Png).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image: glium::texture::RawImage2d<'_, u8> = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

  let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
	texture
}