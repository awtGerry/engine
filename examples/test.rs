use engine::graphics::color::Color;
use engine::graphics::window::Window;
use engine::algorithms::fill::{fill_circle_scanline, fill_triangle, fill_triangle_inundation};

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
            fill_triangle_inundation(100.0, 100.0, 200.0, 200.0, 300.0, 100.0, &Color::new(1.0, 0.0, 0.0));
        }
        window.update();
    }

}
