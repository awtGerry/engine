use engine::graphics::window::Window;
// use engine::graphics::wrapper::*;
// use engine::graphics::shader::Shader;
// use engine::algorithms::lines::draw_dda_line;
use engine::algorithms::curves::draw_infinite_symbol;

// Curvas
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
            draw_infinite_symbol(50.0, 150.0);
            // draw_dda_line(-0.5, -0.5, 0.5, 0.5);
        }
        window.update();
    }

}
