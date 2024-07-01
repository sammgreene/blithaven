use blithaven_rewrite::*;

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut xv = 500;
    let mut yv = 500;
    AppBuilder::new()
        .set_size(2000, 1100)
       .run(move | delta: f32 | {

            text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut
                labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
                laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
                voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                `non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
                [100,100], 40, [1.0,1.0,1.0,1.0]);

            square([500,100], 225, [1.0,0.5,0.5,1.0]);
            square([100,800], 225, [1.0,0.5,0.5,1.0]);

            texture([1000,1000], 400, 400, [1.0,1.0,1.0,1.0], "src/assets/test_image.png");

            x += (xv as f64 * delta) as i32;
            y += (yv as f64 * delta) as i32;
            if x < 0 || x + 225 > 4000 { xv *= -1 }
            if y < 0 || y + 225 > 2200 { yv *= -1 }
            square([x,y], 225, [1.0,1.0,1.0,0.5]);
        })
}
