use engine::graphics::window::Window;
use engine::algorithms::figures::{fill_circle_inundation, fill_rectangle_inundation};

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
            fill_rectangle_inundation(200.0, 150.0, 400.0, 300.0);
            fill_circle_inundation(700.0, 400.0, 50.0);
        }
        window.update();
    }

}
