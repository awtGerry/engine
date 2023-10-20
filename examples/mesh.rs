use engine::graphics::window::Window;
use engine::algorithms::figures::mesh;

fn main()
{
    let mut window = Window::new(800.0, 600.0, "Testing code");
    window.init();

    let mut v1: Vec<f32> = Vec::new();
    let mut v2: Vec<f32> = Vec::new();

    for i in 0..51
    {
        v1.push((30 + i) as f32);
    }
    for i in 0..31
    {
        v2.push((50 + i) as f32);
    }

    while !window.should_close()
    {
        unsafe
        {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            mesh(v1.clone(), v2.clone(), 4.0);
        }
        window.update();
    }

}
