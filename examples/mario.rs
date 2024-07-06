extern crate blithaven_rewrite;
use blithaven_rewrite::*;

const WIDTH: u32 = 3200;
const HEIGHT: u32 = 2000;

const MAP: [[i32; 16]; 10] = [
  [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
  [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
  [0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0],
  [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
  [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
  [1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1],
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
  [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
  [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
];

fn main() {
  let cam_pos = [0.0,2.0];
  let mut player_pos = [0.0,5.0];
  let mut player_vel = [0.0,0.0];

  AppBuilder::new()
    .set_size(WIDTH / 2, HEIGHT / 2)
    .run(move | delta | {
      draw_map(cam_pos);
      draw_player(player_pos, player_vel, cam_pos);


      if key_pressed(VirtualKeyCode::W) {
        if on_ground(player_pos, player_vel, delta) {
          player_vel[1] = 25.0;
          player_pos[1] += 0.01;
        }
      }
      if key_pressed(VirtualKeyCode::D) {
        player_vel[0] = 10.0;
      }
      if key_pressed(VirtualKeyCode::A) {
        player_vel[0] = -10.0;
      }
      if key_pressed(VirtualKeyCode::D) && key_pressed(VirtualKeyCode::A) {
        player_vel[0] = 0.0;
      }
      if on_ground(player_pos, player_vel, delta) && !key_pressed(VirtualKeyCode::D) && !key_pressed(VirtualKeyCode::A) {
        player_vel[0] *= 0.5;
      }
      else {
        player_vel[0] *= 0.95; 
      }

      if !on_ground(player_pos, player_vel, delta) {
        player_vel[1] -= 0.7;
      }
      else {
        player_vel[1] = 0.0;
        player_pos[1] = 1.0;
      }
      player_pos[0] += player_vel[0] * delta;
      player_pos[1] += player_vel[1] * delta;

      circle([1000,1500], 432, [1.0,0.4,0.8,0.7]);
    });
}

fn wto_screen_space(cam_pos: [f64; 2], pos: [f64; 2]) -> [i32; 2] {
  let relative_to_cam = [pos[0] - cam_pos[0], pos[1] - cam_pos[1]];
  let screen_pos = [(relative_to_cam[0] * 200.0) as i32 + WIDTH as i32 / 2, (-relative_to_cam[1] * 200.0) as i32 + HEIGHT as i32 / 2];

  screen_pos
}
fn wx_screen_space(cam_pos: [f64; 2], pos: f64) -> i32 {
  let relative_to_cam = pos - cam_pos[0];
  let screen_pos = (relative_to_cam * 200.0) as i32 + WIDTH as i32 / 2;

  screen_pos
}
fn wy_screen_space(cam_pos: [f64; 2], pos: f64) -> i32 {
  let relative_to_cam = pos - cam_pos[1];
  let screen_pos = (-relative_to_cam * 200.0) as i32 + HEIGHT as i32 / 2;
  
  screen_pos
}


fn x_screen_value(pos: f64) -> i32 {
  let screen_pos = (pos * 200.0) as i32 + WIDTH as i32 / 2;

  screen_pos
}
fn y_screen_value(pos: f64) -> i32 {
  let screen_pos = (-pos * 200.0) as i32 + HEIGHT as i32 / 2;
  
  screen_pos
}

fn world_value(value: f64) -> i32 {
  let value = (value * 200.0) as i32;

  value
}

fn on_ground(player_pos: [f64; 2], player_vel: [f64; 2], delta: f64) -> bool {
  player_pos[1] + (player_vel[1] / 2.0) * delta - 1.0 <= 0.0
}

fn draw_player(player_pos: [f64; 2], player_vel: [f64; 2], cam_pos: [f64; 2]) {
  rect(wto_screen_space(cam_pos, player_pos), world_value(12.0/16.0), world_value(1.0), [1.0,1.0,1.0,0.5]);
  // circle(wto_screen_space(cam_pos, player_pos), 20, [1.0,0.0,0.0,1.0]);
  if player_vel[0] >= 0.0 {
    texture(wto_screen_space(cam_pos, player_pos), world_value(12.0/16.0), world_value(1.0), [1.0,1.0,1.0,1.0], "src/assets/mario_base.png");
  }
  else {
    texture(wto_screen_space(cam_pos, [player_pos[0] + 0.75, player_pos[1]]), world_value(-12.0/16.0), world_value(1.0), [1.0,1.0,1.0,1.0], "src/assets/mario_base.png");
  }
}

fn draw_map(cam_pos: [f64; 2]) {
  for y in 0 .. 10 {
    for x in 0 .. 16 {
      if MAP[y][x] > 0 {
        blit(wto_screen_space(cam_pos, [x as f64 - 8.0, -(y as f64) + 5.0]), world_value(1.0), "src/assets/ground.png");
      }
    }
  }
}
