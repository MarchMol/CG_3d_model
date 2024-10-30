use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use screen::framebuffer;
use obj::Obj;

mod screen;
mod vertex;
mod fragments;
mod obj;

fn main() {
    // Window
    let window_width = 800;
    let window_height = 600;
    let mut window = Window::new(
        "3D modeling - Render Pipeline",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();
    window.set_position(500, 500);
    window.update();

    // Framebuffer
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let frame_delay = Duration::from_millis(16);

    // Obj
    let object =  Obj::load("./spaceship.obj").expect("Failed to load obj");
    let vertex_array = object.get_vertex_array();

    // Main Window Loop:
    while window.is_open() {
        framebuffer.clear();
        if window.is_key_down(Key::Escape) {
            break;
        }
        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
