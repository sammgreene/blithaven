
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2],
    pub style: f32
}

glium::implement_vertex!(Vertex, position, color, uv, style);