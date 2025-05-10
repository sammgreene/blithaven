use blithaven::*;

fn main() {
  AppBuilder::new()
    .set_size(500, 500) // Window size specifier
    .run(move |_delta| { // delta is time since previous frame

      rect([100,100], 100, 100, [1.0,1.0,0.3,1.0]); // positions and units are in pixels

    });
}