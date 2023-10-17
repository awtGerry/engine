use engine::graphics::window::Window;
use engine::algorithms::figures::draw_diamond;

fn main()
{
    let mut window = Window::new(800, 600, "Draw a diamond");
    window.init();

    while !window.should_close()
    {
        unsafe
        {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            draw_diamond(0.0, -0.5, 0.0, 0.5, -0.5, 0.0);
            draw_diamond(0.0, -0.5, 0.0, 0.5, 0.5, 0.0);
        }
        window.update();
    }

}
