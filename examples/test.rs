use engine::graphics::window::Window;
use engine::algorithms::figures::draw_triangle;

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
            draw_triangle(-0.2, -0.4, 0.2, 0.4, 0.0);
        }
        window.update();
    }

}
