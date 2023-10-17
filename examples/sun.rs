use engine::graphics::window::Window;
use engine::algorithms::curves::draw_sun;

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
            draw_sun(100);
        }
        window.update();
    }

}