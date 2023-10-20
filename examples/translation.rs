use engine::algorithms::fill::fill_rectangle;
use engine::graphics::window::Window;
use engine::graphics::shader::Shader;

use std::ffi::CStr;
use cgmath::{Matrix, Matrix4, SquareMatrix, vec3, Rad};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 720;

unsafe fn rotation(x1: f32, y1: f32, x2: f32, y2: f32, shader: &Shader, increment: f32)
{
    gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    let mut translation_matrix: Matrix4<f32> = Matrix4::identity();
    translation_matrix = translation_matrix * Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
    translation_matrix = translation_matrix * Matrix4::<f32>::from_angle_z(Rad(increment));

    shader.bind();
    let loc = gl::GetUniformLocation(shader.id, CStr::from_bytes_with_nul(b"transform\0").unwrap().as_ptr());
    gl::UniformMatrix4fv(loc, 1, gl::FALSE, translation_matrix.as_ptr());

    fill_rectangle(x1, y1, x2, y2);
}

fn main()
{
    let mut window = Window::new(WIDTH, HEIGHT, "Traslation");
    window.init();

    let shader = Shader::new("examples/shaders/transform.vs", "examples/shaders/transform.fs");

    let x1 = 0.5;
    let y1 = 0.5;
    let x2 = -0.5;
    let y2 = -0.5;

    let mut increment = 100.0;

    while !window.should_close()
    {
        unsafe
        {
            rotation(x1, y1, x2, y2, &shader, increment);
            fill_rectangle(x1, y1, x2, y2);
            increment += 0.001;
        }
        window.update();
    }

}
