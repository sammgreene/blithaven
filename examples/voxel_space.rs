extern crate blithaven_rewrite;
use blithaven_rewrite::*;
use image::{io::Reader as ImageReader, GenericImageView};

fn main() {
  let test = ImageReader::open("src/assets/D5.png").unwrap();
  let raw = test.decode().unwrap();
  let heightmap = raw.to_rgba32f();
  let test = ImageReader::open("src/assets/C5W.png").unwrap();
  let raw = test.decode().unwrap();
  let colormap = raw.to_rgba32f();

  let play_pos = [74,74];
  
  AppBuilder::new().set_size(500, 400).run(move | delta | {
    
    for i in (0..20).rev() {
      let line_width = 500 / (i + 1);

      for j in -i .. i {
        println!("{}", (heightmap.get_pixel((play_pos[0] - j) as u32, (play_pos[1] - i) as u32).0[0] * 10000.) as i32);
        line_to_bottom((j + i) * line_width, (heightmap.get_pixel((play_pos[0] - j) as u32, (play_pos[1] - i) as u32).0[0] * 5000.) as i32, line_width, colormap.get_pixel((play_pos[0] - j) as u32, (play_pos[1] - i) as u32).0)
      }
    }
  })
}

fn line_to_bottom(x: i32, y: i32, width: i32, color: [f32; 4]) {
  rect([x, y], width, 800 - y, color)
}
