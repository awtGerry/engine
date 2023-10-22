use engine::graphics::color::Color;
use engine::graphics::window::Window;
// use engine::algorithms::curves::{draw_flower, draw_sun};

fn main()
{
    let mut window = Window::new(800.0, 600.0, "Testing code");
    window.init();

    while !window.should_close()
    {
        unsafe
        {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // draw_sun(800.0, 600.0, 100, 10.0, &Color::new(1.0, 1.0, 0.0));
            // draw_flower(800.0, 600.0, 8.0, &Color::new(1.0, 0.0, 0.0));
        }
        window.update();
    }

}
