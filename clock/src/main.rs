use engine::graphics::window::Window;

mod figures;
mod buildings;
mod textures;
mod clock;

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
<<<<<<< HEAD

            gl::BindTexture(gl::TEXTURE_2D, texture);
            new_shader.bind();
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(vao);

=======
            figures::draw_circle(0.5, 0.0, 0.4);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            new_shader.bind();
            gl::BindVertexArray(vao);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
>>>>>>> e2f530f (Better tree)
            buildings::draw_buildings();
            clock::draw_clock();
        }
        window.update();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }
}
