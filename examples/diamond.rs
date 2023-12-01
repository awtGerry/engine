use engine::graphics::window::Window;
use engine::algorithms::figures::draw_diamond;

fn main()
{
    let mut window = Window::new(800.0, 600.0, "Draw a diamond");
    window.init();

    let number: f32 = 0;

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
