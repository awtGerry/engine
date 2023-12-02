use engine::graphics::window::Window;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

fn main()
{
    let mut window = Window::new(WIDTH, HEIGHT, "Engine");
    window.init();

    while !window.should_close()
    {
        window.update();
    }
}
