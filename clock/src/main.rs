use engine::graphics::window::Window;
mod figures;
mod buildings;
mod textures;
use std::ptr;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 768;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Clock");

    window.init();
    let (new_shader, vbo, vao, ebo, texture) = textures::bg_texture();
    while !window.should_close() {
        unsafe {
            // Blue sky background
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.update();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }
}
