use engine::graphics::window::Window;
use engine::graphics::wrapper::*;
use engine::algorithms::figures::draw_rectangle;
use engine::algorithms::raw_attributes::set_vao_vbo;

use gl::types::*;
use std::mem;

fn main() {
    let mut window = Window::new(1000, 768, "Translation");

    window.init();

    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);

    let mut scale: f32 = 0.0;
    let mut delta: f32 = 0.005;
    scale += delta;
    if scale >= 1.0 || scale <= -1.0 {
        delta *= -1.0;
    }

    let translation: [f32; 16] = [
        1.0, 0.0, 0.0, scale*2 as f32,
        0.0, 1.0, 0.0, scale,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];

    let vertices: [GLfloat; 9] = [
        -1.0, -1.0, 0.0,
        1.0, -1.0, 0.0,
        0.0,  1.0, 0.0,
    ];
    set_vao_vbo(&vao, &vbo, &vertices, 2);

    
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UniformMatrix4fv(0, 1, gl::TRUE, &translation[0]);
            draw_rectangle(-0.5, -0.5, 0.5, 0.5);
            gl::DrawArrays(gl::LINES, 0, 5);
        }

        window.update();
    }
}
