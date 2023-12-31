use engine::graphics::window::Window;
#[allow(unused_imports)]
use engine::algorithms::curves::{draw_curve, draw_curve_8_points, draw_infinity};

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
            draw_infinity(0.0, 0.5, 80.0);
            draw_curve(400.0, 300.0);
            // draw_curve_8_points(0.0, 0.5);
        }
        window.update();
    }

}
