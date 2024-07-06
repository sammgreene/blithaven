mod shaders;
mod batches;
mod vertex;
mod state;
mod builder;
mod app;
mod font;

pub use winit::event::ButtonId;

pub use crate::app::AppBuilder;

pub fn version() {
    println!("Blithaven: 2.0");
}

pub fn square(position: [i32; 2], width: i32, color: [f32; 4]) {
    app::get_app().add_texture_quad(position, width, Some(width), color, "rectangle".to_string());
}
pub fn rect(position: [i32; 2], width: i32, height: i32, color: [f32; 4]) {
    app::get_app().add_texture_quad(position, width, Some(height), color, "rectangle".to_string());
}
pub fn circle(position: [i32; 2], radius: i32, color: [f32; 4]) {
    app::get_app().add_texture_quad([position[0] - radius / 2, position[1] - radius / 2], radius, Some(radius), color, "circle".to_string());
}
pub fn blit(position: [i32; 2], width: i32, path: &str) {
    app::get_app().add_texture_quad(position, width, None, [1.0,1.0,1.0,1.0], String::from(path));
}
pub fn texture(position: [i32; 2], width: i32, height: i32, color: [f32; 4], path: &str) {
    app::get_app().add_texture_quad(position, width, Some(height), color, String::from(path));
}
pub fn text(string: &str, position: [i32; 2], size: i32, color: [f32; 4]) {
    let mut i = 0;
    let height = (size as f32 * 1.5) as i32;
    let mut line = 0;
    for char in string.chars() {
        if char.is_ascii() {
            app::get_app().add_texture_quad([position[0] + i, position[1] + (line * height)], size, Some(height), color, char.to_string());
            i += size;
        }
        if char == '\n' {
            line += 1;
            i = 0;
        }
    }
}

pub fn key_pressed(key: ButtonId) -> bool {
    app::get_app().pressed_keys().contains(&key)
}
pub fn key_press(key: ButtonId) -> bool {
    app::get_app().key_presses().contains(&key)
}
pub fn key_event(key: ButtonId) -> bool {
    app::get_app().key_events().contains(&key)
}