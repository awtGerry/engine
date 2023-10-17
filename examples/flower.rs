use engine::graphics::window::Window;
// use engine::graphics::wrapper::*;
// use engine::graphics::shader::Shader;
use engine::algorithms::curves::{draw_infinity, draw_curve, draw_flower, draw_humito, draw_sun};

fn main()
{
    let mut window = Window::new(800, 600, "Testing code");
    window.init();

    while !window.should_close()
    {
        unsafe
        {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // draw_infinity(0.0, 0.5, 80.0);
            // draw_curve(0.0, 0.5);
            draw_flower(0.0, 0.5);
            // draw_humito(0.0, 0.0);
        }
        window.update();
    }

}
