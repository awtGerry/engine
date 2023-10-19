/* TRANSLATION
 * 
 * This is a test file for the translation algorithm.
 * It will move a figure from one point to another.
 *
*/

use std::ffi::CStr;

use cgmath::{Matrix, Matrix4, SquareMatrix, vec3, Rad};
// use engine::algorithms::figures::fill_rectangle_inundation;
use engine::graphics::window::Window;
use engine::graphics::shader::Shader;
use engine::algorithms::raw_attributes::set_vao_vbo;
use engine::graphics::wrapper::{Vao, Buffer};
// use engine::algorithms::transformations::*;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 720;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Traslation");
    window.init();

    let mut increment = 100.0;

    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);

    let shader = Shader::new("examples/shaders/transform.vs", "examples/shaders/transform.fs");

    let vertices: [f32; 9] = [
        0.0, 0.0, 0.0,
        0.0, -0.5, 0.0,
        -0.5, 0.0, 0.0
    ];

    set_vao_vbo(&vao, &vbo, &vertices, 3);

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let mut translation_matrix: Matrix4<f32> = Matrix4::identity();
            translation_matrix = translation_matrix * Matrix4::<f32>::from_translation(vec3(0.5, -0.5, 0.0));
            translation_matrix = translation_matrix * Matrix4::<f32>::from_angle_z(Rad(increment));

            shader.bind();
            let loc = gl::GetUniformLocation(shader.id, CStr::from_bytes_with_nul(b"transform\0").unwrap().as_ptr());
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, translation_matrix.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            increment += 0.01;
        }
        window.update();
    }

}
